# Implementation status

Last updated: 2026-11-18. This file reports the code that exists and the checks that were run;
it deliberately does not claim unverified integrations.

## Build state

| Component | State | Evidence |
| --- | --- | --- |
| `belhana-common` | ✅ | 8 unit tests; money is integer piasters |
| `belhana-db` | ✅ | pool + embedded-ish SQLx migrations (11 files, incl. `0011_guidance_and_purchasing`) |
| `belhana-auth` | ✅ | 2 tests; Argon2 + JWT access/refresh rotation |
| `belhana-core` | ✅ | 13 tests; domains implemented against SQLite (incl. package-price tests) |
| `belhana-ai` | ✅ | provider abstraction, tool schemas, proposal guardrails; instruction rules in system prompt; no live API key tested |
| `apps/api` | ✅ | ~85 route declarations in 17 modules; startup migrate + seed; healthz |
| `apps/web` | ✅ | 16 Svelte screens (incl. meal tips), Arabic RTL/English, static build + browser E2E pass |
| `apps/desktop` | 🟡 | Tauri 2 shell compiles (`cargo check`); WebView2 required to launch the Windows binary |
| `workers/maintenance_worker` | ✅ | `--once` sweep ran clean (stale prices, expiry reminders, pruning) |

## MVP checklist (master prompt §25)

1. ✅ Register/login; access token + rotating refresh; same household from multiple devices via API.
2. ✅ Household city/currency/size; members with preferences and exclusions.
3. ✅ Member likes/dislikes/exclusions/allergies and structured household rules.
4. ✅ Weekly/monthly budgets and shopping frequency; deterministic what-if.
5. ✅ Products, categories, units and merchants (incl. Olad Zain).
6. ✅ Manual price observations with dated history.
7. ✅ Paste a post → parse preview → unreviewed batch → review screen.
8. ✅ Ambiguous prices (unit/date/product) remain unreviewed and out of verified costing.
9. ✅ Recipes with deterministic cost; unknown prices shown as unknown.
10. ✅ Pantry/fridge/freezer lots and movements, valuation and expiry reminders.
11. ✅ 7-day breakfast/lunch/dinner plan within rules/budget with conflicts and explanations.
12. ✅ Consolidated shopping list with pack-size rounding and freshness flags.
13. ✅ Purchase flow records actual price, updates stock and creates price evidence.
14. ✅ Occasion menus by guest count/style/budget with shopping list and actual-vs-planned report.
15. ✅ AI suggests/explains via tool loop when a provider key is configured; mutations are proposals.
    Without a key the app stays fully functional in manual mode.
16. ✅ Arabic default with English toggle (reactivity verified in the browser).
17. 🟡 Web build + Tauri shell delivered; running the desktop binary depends on local WebView2 tooling.
18. ✅ Household rules for AI instruction sentences, meal staples (breakfast/lunch/dinner), fixed-spend
    purchases and leftover carry-over; six defaults seeded per household.
19. ✅ Manual/package price entry and editing with audit (`previous_amount_minor`, `edited` event) and
    per-unit cheapest-price comparison that never mixes units.
20. ✅ Leftover carry-over lots (default on, FEFO-first use) and a filterable meal-tips page.
21. ✅ Fixed-spend shopping lines (e.g. 20 EGP of salt) that record expense but create no fake stock lot.
22. ✅ Meal search across names, ingredients and tags, with web-search/add-it-yourself fallback for
    missing dishes, local star bookmarks plus a saved-only filter, and sub-kilogram weights shown in grams.

## Meal search and local saving (2026-11-18)

`GET /api/recipes?search=` now searches both recipe names, the description, ingredient raw names
(`recipe_ingredients.raw_name_ar/en`) and tags, and ignores blank queries. The `/recipes` screen
debounces requests, discards out-of-order responses, and falls back when nothing matches: it can open
a Google search for the dish or prefill the new-recipe form so the meal is created locally. Any recipe
can be starred; bookmarks live in `localStorage` under `belhana.savedRecipes`, and a saved-only filter
lists them after a reload. `formatQty` renders weights under 1 kg in grams and removes floating-point
noise on recipes, shopping, inventory, dashboard, plan and batch-cooking screens.

Verified against the seeded API: `كشري` → 1 result, `طماطم` (an ingredient) → 16, nonsense → 0, empty
query → 34. `cargo test --workspace` passes 23 tests and the Svelte production build succeeds.

## Recipe content expansion (2026-11-18)

The shared recipe library now contains 34 dishes: the original 18 plus 16 famous Egyptian dishes
(رقاق باللحمة، حواوشي، مكرونة بشاميل، كفتة مشوية، لوبيا باللحمة، صينية بطاطس باللحمة، كبدة إسكندراني،
سمك سنجاري، محشي كرومب، فتة لحمة، بسبوسة، أم علي، كنافة، قطايف، مهلبية، فطير مشلتت). Every system
recipe now has a full method with practical times, heat levels, doneness cues and storage notes, written
originally for Belhana. Seven specialty products (سميد، جوز هند، مكسرات مشكلة، كنافة، قطايف، نشا،
فانيليا) were added so the new dishes map to catalog products for costing and shopping.

Existing system recipes are refreshed in place on startup: description, timing, cooking method and
ordered steps are replaced, while ingredients and their costing stay untouched. Verified after the
change: 34 recipes / 34 unique names, 130 products / 130 unique, steps present for new and existing
dishes, and counts unchanged after a second API restart.

## Guidance-feature verification pass (2026-11-17)

`python scripts/smoke_guidance.py` passed 30/30 checks: 6 default rules with the expected types and
params, 25 tips / 9 categories, carry-over default/toggle/list, package prices and cheapest-per-unit
selection, price edit with retained prior value, staple/leftover explanation text, fixed-spend shopping
line, no `spend` stock lot on trip completion, and the `edited` audit event. Browser checks on
`/more/tips`, `/more/rules`, `/more/prices`, `/inventory` and `/shopping` showed no console errors, and
the English toggle switched page content and bottom-nav labels. Shopping quantities are rounded for
display (`1.6 kg`). `cargo test --workspace` passes 23 tests.

A full-screen preview also caught a recipe-seeding bug: `create_recipe` wrote `is_system = 0` while the
seed guard checked `is_system = 1`, so each startup re-inserted the system library (216 rows in the dev
DB). Household-less recipes are now marked as system recipes and the guard matches `household_id IS NULL`.
After deduplicating the dev DB (repointing plan/event references first) the API returns 18 unique recipes
and still returns 18 after a second restart; the plan page resolves saved meals normally.

## Final verification pass (2026-11-17)

Browser E2E against the Vite dev server and seeded API verified: login → dashboard → bottom navigation →
refresh keeps the session → `/plan`, `/shopping`, `/inventory`, `/recipes`, all nine `/more` subpages,
imports batch review, and the Arabic/English toggle, with no console errors. Defects found there were
fixed: Svelte 5 `children` prop in the layout; auth restored before child `onMount` so a hard refresh
no longer bounces to login; language switching now re-keys page content; and
`GET /api/imports/batches/{id}/source-post` was added for the review screen's original-post view.
Also corrected the real `Money` `Add`/`Sub` implementations (explicit integer arithmetic), removed a
duplicate Arabic unit pattern and a dead assignment, and generated `icon.ico` so the Tauri crate checks
cleanly. (At MVP delivery `cargo test --workspace` passed 20 tests; it now passes 23 after phase 13.)

## Known limitations / open work

- SQLite is the active database. Migrations use portable SQL, but a Postgres connection pool and a
  migration verification pass are still required before production deployment.
- No live Gemini/OpenAI key was available; provider response shapes are based on documented APIs and
  should be smoke-tested with a real key before enabling in production.
- Facebook/Meta API, OCR import, CSV/Excel import, Android/iOS builds and offline sync are not built.
- Frontend has non-blocking Svelte accessibility warnings (form labels not programmatically bound).
- `JWT_SECRET` defaults to a development value; must be overridden in any deployment.

## Decision / constraint notes

- The master prompt's 19-crate layout was consolidated into 5 libraries + API + worker.
- No Tailwind; CSS variables in `apps/web/src/app.css` instead.
- All money values in JSON and DB are integer minor units (piasters). The UI divides by 100 only for display.
