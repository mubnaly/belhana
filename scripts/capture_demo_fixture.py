"""Capture a seeded API snapshot for the backend-free GitHub Pages demo.

Run only during development while `belhana-api` is running on 127.0.0.1:8080:
    python scripts/capture_demo_fixture.py

The output is committed as apps/web/src/lib/demo-data.json and served by the
in-browser demo client, so the static site needs no backend.
"""

from datetime import date
from pathlib import Path
import json
import time

import requests

BASE = "http://127.0.0.1:8080"
OUT = Path(__file__).resolve().parents[1] / "apps" / "web" / "src" / "lib" / "demo-data.json"

GET_PATHS = [
    "/api/dashboard",
    "/api/household",
    "/api/catalog/products?limit=200",
    "/api/catalog/categories",
    "/api/merchants",
    "/api/prices?limit=100",
    "/api/prices?limit=100&review_status=confirmed",
    "/api/prices?limit=100&review_status=verified",
    "/api/imports/batches",
    "/api/recipes?limit=200",
    "/api/batch-guides",
    "/api/meal-tips",
    "/api/meal-tips/categories",
    "/api/nutrition-guide",
    "/api/rules",
    "/api/family/members",
    "/api/family/exclusions",
    "/api/budget/status?period=weekly",
    "/api/budget/history?days=30",
    "/api/inventory/lots",
    "/api/inventory/locations",
    "/api/inventory/expiring?days=7",
    "/api/inventory/valuation",
    "/api/inventory/leftovers",
    "/api/events",
    "/api/meal-plans",
    "/api/shopping-lists/active",
    "/api/ai/status",
    "/api/ai/proposals?status=pending",
]


def wait_for_api():
    for _ in range(40):
        try:
            if requests.get(f"{BASE}/healthz", timeout=1).ok:
                return
        except requests.RequestException:
            pass
        time.sleep(0.25)
    raise SystemExit("API not reachable on 127.0.0.1:8080")


def main():
    wait_for_api()
    s = requests.Session()
    email = "demo@belhana.app"
    r = s.post(
        f"{BASE}/api/auth/register",
        json={
            "email": email,
            "password": "demo123",
            "display_name": "أسرة بالهنا",
            "household_name": "الأسرة التجريبية",
        },
        timeout=10,
    )
    if r.status_code >= 400:
        r = s.post(f"{BASE}/api/auth/login", json={"email": email, "password": "demo123"}, timeout=10)
    r.raise_for_status()
    auth = r.json()
    s.headers.update({"Authorization": f"Bearer {auth['access_token']}"})

    # A weekly budget makes the dashboard/budget screens useful in demo mode.
    s.put(
        f"{BASE}/api/household",
        json={"household_size": 4, "weekly_budget_minor": 150000},
        timeout=10,
    ).raise_for_status()

    # Seed a week of meals and a shopping list so the dashboard, plan and
    # shopping screens have content on first load.
    today = date.today().isoformat()
    plan = s.post(
        f"{BASE}/api/meal-plans",
        json={
            "days": 7,
            "servings_per_meal": auth.get("household", {}).get("household_size") or 4,
            "start_date": today,
            "periods_enabled": ["breakfast", "lunch", "dinner"],
            "save": True,
        },
        timeout=20,
    )
    plan.raise_for_status()
    plan_id = plan.json()["plan_id"]
    shopping = s.post(f"{BASE}/api/shopping-lists", json={"plan_id": plan_id}, timeout=20)
    if not shopping.ok:
        print(f"shopping list: {shopping.status_code} (an active list may already exist)")

    fixture = {"auth": auth, "data": {}, "recipes": {}, "cost": {}}
    for path in GET_PATHS:
        res = s.get(f"{BASE}{path}", timeout=20)
        if res.ok:
            fixture["data"][path] = res.json()
        else:
            print(f"skip {path}: {res.status_code}")

    plans = fixture["data"].get("/api/meal-plans", [])
    if plans:
        path = f"/api/meal-plans/{plans[0]['id']}"
        res = s.get(f"{BASE}{path}", timeout=10)
        if res.ok:
            fixture["data"][path] = res.json()

    for recipe in fixture["data"].get("/api/recipes?limit=200", []):
        rid = recipe["id"]
        detail = s.get(f"{BASE}/api/recipes/{rid}", timeout=10)
        if not detail.ok:
            continue
        fixture["recipes"][rid] = detail.json()
        servings = fixture["recipes"][rid].get("default_servings") or 4
        cost = s.get(f"{BASE}/api/recipes/{rid}/cost?servings={servings}", timeout=10)
        if cost.ok:
            fixture["cost"][f"{rid}|{servings}"] = cost.json()

    batches = fixture["data"].get("/api/imports/batches", [])
    if batches:
        bid = batches[0]["id"]
        for suffix in ["", "/observations", "/source-post"]:
            path = f"/api/imports/batches/{bid}{suffix}"
            res = s.get(f"{BASE}{path}", timeout=10)
            if res.ok:
                fixture["data"][path] = res.json()

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(fixture, ensure_ascii=False, separators=(",", ":")), encoding="utf-8")
    print(f"wrote {OUT.relative_to(OUT.parents[3])} ({OUT.stat().st_size // 1024} KB)")


if __name__ == "__main__":
    main()
