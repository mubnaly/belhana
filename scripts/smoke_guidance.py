"""End-to-end smoke test for the guidance / purchasing feature set (0011).

Run against an API that has applied migration 0011 and seeded defaults.
Usage:  python scripts/smoke_guidance.py
"""
import json
import os
import sqlite3
import sys
import tempfile
import time
import urllib.error
import urllib.request
from datetime import datetime

BASE = "http://127.0.0.1:8080"
DB = os.path.join(tempfile.gettempdir(), "opencode", "belhana_smoke.db")
HERE = os.path.dirname(os.path.abspath(__file__))

try:
    sys.stdout.reconfigure(encoding="utf-8")
except AttributeError:
    pass

_passed = 0
_failed = 0


def check(name, cond, detail=""):
    global _passed, _failed
    if cond:
        _passed += 1
        print(f"PASS  {name}" + (f" - {detail}" if detail else ""))
    else:
        _failed += 1
        print(f"FAIL  {name}" + (f" - {detail}" if detail else ""))


TOKEN = None


def api(method, path, body=None):
    data = None
    headers = {}
    if TOKEN:
        headers["Authorization"] = f"Bearer {TOKEN}"
    if body is not None:
        data = json.dumps(body).encode("utf-8")
        headers["Content-Type"] = "application/json; charset=utf-8"
    req = urllib.request.Request(BASE + path, data=data, headers=headers, method=method)
    with urllib.request.urlopen(req, timeout=30) as resp:
        raw = resp.read()
        return json.loads(raw.decode("utf-8")) if raw else None


def main():
    global TOKEN

    # health + auth
    health = api("GET", "/healthz")
    check("health", health.get("ok") is True, f"app={health.get('app')}")

    email = f"smoke_{datetime.now():%Y%m%d%H%M%S}@belhana.app"
    reg = api("POST", "/api/auth/register", {
        "email": email, "password": "secret123",
        "display_name": "Smoke", "household_name": "Smoke Family",
    })
    TOKEN = reg.get("access_token")
    check("register", bool(TOKEN), f"household={reg.get('household', {}).get('id')}")

    # rules
    rules = api("GET", "/api/rules")
    types = {r["rule_type"] for r in rules}
    check("rules seeded", len(rules) >= 6, f"count={len(rules)}")
    for t in ("ai_instruction", "meal_staple", "fixed_spend", "leftover_carryover"):
        check(f"rule {t} present", t in types)
    ai = next(r for r in rules if r["rule_type"] == "ai_instruction")
    ai_sentence = json.loads(ai["params"]).get("sentence", "")
    check("ai_instruction sentence", len(ai_sentence) > 10, f"len={len(ai_sentence)}")
    staples = [r for r in rules if r["rule_type"] == "meal_staple"]
    periods = {json.loads(r["params"]).get("period") for r in staples}
    check("staple periods", {"breakfast", "lunch", "dinner"} <= periods, f"periods={sorted(periods)}")
    spend = next(r for r in rules if r["rule_type"] == "fixed_spend")
    check("fixed_spend 2000", json.loads(spend["params"]).get("amount_minor") == 2000)

    # tips
    tips = api("GET", "/api/meal-tips")
    cats = api("GET", "/api/meal-tips/categories")
    check("meal tips seeded", len(tips) >= 25, f"count={len(tips)}")
    check("tip categories", {"rice", "bread", "spices", "leftovers"} <= set(cats), f"cats={cats}")
    spice_tips = api("GET", "/api/meal-tips?category=spices")
    check("spices filter", len(spice_tips) >= 3, f"count={len(spice_tips)}")

    # leftovers
    products = api("GET", "/api/catalog/products?limit=200")
    left0 = api("GET", "/api/inventory/leftovers")
    check("leftovers baseline", len(left0) == 0, f"count={len(left0)}")
    some = products[0]
    lot = api("POST", "/api/inventory/lots", {
        "product_id": some["id"], "quantity": 5, "unit": "kg", "notes": "smoke leftover",
    })
    check("new lot carries over by default", lot["carry_over"] is True)
    check("leftover appears", len(api("GET", "/api/inventory/leftovers")) == len(left0) + 1)
    api("POST", f"/api/inventory/lots/{lot['id']}/carry-over", {"carry_over": False})
    check("carry-over opt-out removes", len(api("GET", "/api/inventory/leftovers")) == len(left0))
    api("POST", f"/api/inventory/lots/{lot['id']}/carry-over", {"carry_over": True})
    check("carry-over opt-in restores", len(api("GET", "/api/inventory/leftovers")) == len(left0) + 1)

    # prices: packages, edit, cheapest per-unit
    eggs = next(p for p in products if p.get("name_en") == "Eggs")
    check("eggs product", bool(eggs["id"]), f"default_unit={eggs.get('default_unit')}")
    common = {
        "product_id": eggs["id"], "unit": "piece", "package_unit": "piece",
        "source_type": "manual", "unit_status": "confirmed", "observation_date": "2026-11-17",
    }
    o30 = api("POST", "/api/prices", dict(common, amount_minor=13000, package_size=30, notes="30 eggs 130"))
    o15 = api("POST", "/api/prices", dict(common, amount_minor=6000, package_size=15, notes="15 eggs 60"))
    check("package fields stored", o30["package_size"] == 30 and o30["package_unit"] == "piece",
          f"size={o30['package_size']}")
    api("POST", f"/api/prices/{o30['id']}/review", {"action": "verify"})
    api("POST", f"/api/prices/{o15['id']}/review", {"action": "verify"})
    best = api("GET", f"/api/prices/best/{eggs['id']}")
    check("cheapest per-unit wins", best["id"] == o15["id"],
          f"best_amount={best['amount_minor']} pack={best['package_size']}")
    edited = api("PUT", f"/api/prices/{o30['id']}", {
        "amount_minor": 15000, "unit": "piece", "package_size": 30,
        "package_unit": "piece", "observation_date": "2026-11-17",
    })
    check("edit amount", edited["amount_minor"] == 15000, f"amount={edited['amount_minor']}")
    check("previous amount retained", edited["previous_amount_minor"] == 13000,
          f"prev={edited['previous_amount_minor']}")

    # plan explanations: staple fit + leftovers.
    # First preview a plan, then put a needed ingredient into stock so the
    # re-run planner can report that it is reusing leftovers.
    preview = api("POST", "/api/meal-plans", {"days": 2, "save": False})
    target = None
    for slot in preview["plan"]["slots"]:
        for missing in slot.get("missing", []):
            target = missing
            break
        if target:
            break
    check("preview has missing item", target is not None)
    if target:
        api("POST", "/api/inventory/lots", {
            "product_id": target["product_id"], "quantity": 100, "unit": target["unit"],
            "notes": "smoke planned leftover",
        })
    plan = api("POST", "/api/meal-plans", {"days": 2, "save": False})
    explain = " ".join(s.get("explanation", "") for s in plan["plan"]["slots"])
    exp_staple = open(os.path.join(HERE, "expect_staple.txt"), encoding="utf-8").read().strip()
    exp_left = open(os.path.join(HERE, "expect_leftover.txt"), encoding="utf-8").read().strip()
    check("staple in explanation", exp_staple in explain)
    check("leftover in explanation", exp_left in explain)

    # shopping: fixed-spend line + no fake stock lot on completion
    saved = api("POST", "/api/meal-plans", {"days": 2, "save": True})
    listing = api("POST", "/api/shopping-lists", {"plan_id": saved["plan_id"]})
    fixed = [i for i in listing["items"] if i.get("buy_by_amount_minor") == 2000]
    check("fixed-spend line generated", len(fixed) >= 1, f"lines={len(fixed)}")
    if fixed:
        api("POST", f"/api/shopping-lists/{listing['id']}/items/{fixed[0]['id']}",
            {"purchased": True, "actual_price_minor": 2000})
    lots_before = len(api("GET", "/api/inventory/lots"))
    api("POST", f"/api/shopping-lists/{listing['id']}/complete", {})
    lots_after = api("GET", "/api/inventory/lots")
    spend_lots = [l for l in lots_after if l["unit"] == "spend"]
    check("no fake spend stock lot", len(spend_lots) == 0)
    check("no stock created for fixed-spend only", len(lots_after) == lots_before,
          f"before={lots_before} after={len(lots_after)}")

    # audit event for the price edit
    conn = sqlite3.connect(DB)
    n = conn.execute(
        "SELECT COUNT(*) FROM price_review_events WHERE observation_id=? AND action='edited'",
        (o30["id"],),
    ).fetchone()[0]
    conn.close()
    check("edited audit event", n >= 1, f"events={n}")

    print(f"\nRESULT: {_passed} passed, {_failed} failed")
    return 1 if _failed else 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except Exception as exc:  # noqa: BLE001
        print(f"ERROR: {type(exc).__name__}: {exc}")
        sys.exit(2)
