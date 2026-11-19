# Implementation status

Last updated: 2026-11-19. This file reports the code that exists and the checks that were run;
it deliberately does not claim unverified integrations.

## Build state

| Component | State | Evidence |
| --- | --- | --- |
| `belhana-common` | ✅ | 8 unit tests; money is integer piasters |
| `belhana-db` | ✅ | pool + embedded-ish SQLx migrations (13 files, incl. `0011_guidance_and_purchasing`, `0012_product_nutrition`, `0013_product_macros`) |
| `belhana-auth` | ✅ | 2 tests; Argon2 + JWT access/refresh rotation |
| `belhana-core` | ✅ | 30 tests; domains implemented against SQLite (incl. package-price, macro, MET/BMR and guidance tests) |
| `belhana-ai` | ✅ | provider abstraction, tool schemas, proposal guardrails; instruction rules in system prompt; no live API key tested |
| `apps/api` | ✅ | 86 route declarations in 17 modules (incl. nutrition guide + athlete plan); startup migrate + seed; healthz |
| `apps/web` | ✅ | 18 app screens (plus login/register), Arabic RTL/English, theme switch in Settings, light/dark sticky-note themes, monochrome SVG icons, static build clean (no a11y warnings) + browser E2E pass |
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
23. ✅ Recipes reachable from the bottom navigation; the home page shows a structured daily table
    (breakfast, morning snack, lunch, afternoon snack, dinner) with bread, side dishes, rice/salad and
    snack components, plus quick homemade basics such as bread.
24. ✅ Estimated calories on every product/item and meal — recipes and their ingredients, plan slots,
    today's table, quick basics, shopping items and inventory/stock lots — computed from editorial
    per-product nutrition and always labelled as estimates (`≈` / `kcalEstimate`).
25. ✅ Estimated protein/carbs/fat/fibre on recipes, plans and shopping; researched diet and athlete
    guidance on two dedicated pages; MET burn and macro targets computed with clear estimate labels.

## Macros, diet & athlete guidance, theme in Settings (2026-11-19)

Research was recorded before coding in `docs/19-NUTRITION-AND-SPORT-EVIDENCE.md` with source URLs.
Migration `0013_product_macros` adds editorial per-100 g protein, carbs, fat and fibre to `products`,
refreshed idempotently by the seeder. `belhana_core::nutrition` derives macro estimates for recipes,
plan slots, today's table, shopping items and stock lots; `belhana_core::guidance` holds bilingual
micronutrient, diet, exercise, meal-timing and source content plus an athlete-plan calculator. All
nutrition, burn and target numbers are prefixed with `≈` and called estimates; medical-supervision
cautions remain on elimination diets and deficiency advice.

`GET /api/nutrition-guide` and `POST /api/nutrition/athlete-plan` back two new `/more` pages:
`/more/diets` (micronutrients, Mediterranean/elimination/balanced-family plans, sources) and
`/more/athletes` (Mifflin–St Jeor BMR, activity/goal factors, ISSN-style protein target, MET burn,
pre/post-workout timing and alternatives). The floating theme button was removed; the only colour
control is now a Light/Dark card on `/more/settings`, keeping `belhana.theme` persistence and the
early first-paint restore.

Verified: `cargo check --workspace` and `cargo test --workspace` (30 tests) clean; `npm run build`
clean. API smoke on a fresh DB: guide 200 (3 diets, 7 micronutrients, 10 exercises, 4 timing
sections, 10 sources); athlete plan for 70 kg male 200 with `bmr=1649`, `tdee=2556`, `protein_g=98`,
`carbs_g=335.4`, `fat_g=91.4`, `burn_kcal=245`, `met=3.5`; 70 kg examples corrected to ≈245 kcal/h
brisk walking and ≈245–420 kcal/h lifting, not 300/900. Recipes, dashboard courses, plan slots
(3/3) and shopping items (13/14 with a product match) all returned `estimated_macros`. Browser pass
on all 18 screens in Arabic/English and light/dark: both `/more` tiles visible, 0 console errors,
no emoji, no `.theme-toggle`, `currentColor` icons, no slot underlines (the two `dashed` elements are
intentional `.notice` callout borders), athlete recalculation on 80 kg (`bmr=1749`, `burn=280`),
diet tabs/accordions/sources, language toggles, and Settings theme switching with persistence.
Responsive sticky-note rules remain in `app.css` (`@media (max-width: 560px)` and `min-width: 900px`).

## Estimated calories, light/dark notes & monochrome icons (2026-11-19)

Migration `0012_product_nutrition` adds `kcal_per_100g` and `grams_per_unit` to `products`, seeded from
category defaults with named overrides in an idempotent refresh. `belhana_core::nutrition`
(`grams_for`, `kcal_for`, `kcal_from_row`, `recipe_kcal_map`) derives calories from ingredient
quantities and product nutrition rather than storing them as verified facts; recipe calories are
computed, not asserted. UI surfaces (`kcalText()` in `format.js`) prefixes every value with `≈` and an
“estimated” label so no invented nutrition is presented as measured.

The web now ships **two** matching cartoon “notes on the wall” themes — a warm-cream light palette and
a charcoal/teal-plus-amber dark palette (not generic purple). Both keep the same sticky-note language:
dotted wall, taped/tilted notes, ruled lines, sticker chips and offset shadows. Today's table is a
responsive `auto-fit` grid of side-by-side note cards; all note cards reflow from phone to desktop, and
plan slots no longer have dashed underlines. The theme toggle persists `belhana.theme` in
`localStorage`, restored early in `app.html`; it now lives only on `/more/settings` (see below).
`Icon.svelte` provides single-color inline SVGs (all paths
use `currentColor`) and has replaced every colored emoji across all screens; like/dislike preferences
use labelled icon buttons.

Verified: API smoke check (recipes 5/5 with kcal; dashboard course order breakfast → morning snack →
lunch → afternoon snack → dinner; shopping 12 items / 11 with kcal; inventory lot
`estimated_kcal = 2040`; plan 6/6 slots with kcal); browser checks on `/`, `/recipes`, `/plan`,
`/shopping`, `/inventory` and `/more` in both themes (no emoji, no dashed borders, 0 bad icons, no
console errors, estimate labels present, responsive note grid, recipe-modal ingredient kcal visible).
`cargo test --workspace` passes 23 tests and the Svelte production build is clean with no warnings.

## Dark wall theme and recent-price cleanup (2026-11-19)

The dark cartoon theme introduced here — a deep dotted "wall" background, sticky-note cards taped at
the top with a small tilt, ruled paper lines, hard offset shadows, outlined sticker buttons, marker
strokes under section titles, dark inputs and a dark bottom navigation — was later joined by a
matching light theme and a user toggle (see the estimated-calories section above). Cards straighten
and transitions stop under `prefers-reduced-motion`. All screens share the global `app.css`.

The dashboard's recent-prices block now keeps only the latest observation per product, so repeated
entries for the same item no longer fill the list. Units and review status are localized in Arabic
(`قطعة`، `كجم`، `مؤكد`)، while English keeps the raw labels.

Verified with browser style checks on `/`, `/more` and the recipe modal (wall background `#171320`,
note cards `#2c2536`, taped top-level cards, dark modal and overlay, no console errors). `cargo test
--workspace` passes 23 tests and the Svelte production build succeeds.

## Daily table, recipes nav and quick basics (2026-11-19)

`/api/dashboard` now returns `today_table`, ordered as breakfast → morning snack → lunch → afternoon snack
→ dinner. A planned meal from today's plan becomes the main dish; otherwise a deterministic library
suggestion is shown and flagged «مقترح». Each meal is completed with components: breakfast gets bread and
at least two adamas, lunch gets rice (skipped when the main is already a rice dish) and a green salad,
dinner gets bread and one adama, and fruit/vegetable snacks fill the gaps between meals. `quick_basics`
surfaces easy homemade items (bread, white rice, green salad, an adama and the two snacks) on the home
page.

Recipes moved into the bottom navigation after Plan (six compact items). `/recipes?open=<id>` opens the
detail modal directly, so every dish in the home table is one tap from its full recipe.

Eight quick component recipes were added with empty `meal_types` so the planner never selects them as
standalone meals: خبز بلدي سريع (one short 20–30 minute proof), أرز أبيض, سلطة خضراء, جبنة بيضاء بالزيت,
بيض مسلوق, زبادي بالخيار, سناك فواكه and سناك خضار. The missing `خميرة` product was added to the catalog,
bringing the shared library to 42 recipes. System recipe refresh now also updates `meal_types` and tags
idempotently, so existing databases receive the new `bread`, `adama`, `salad`, `rice` and snack tags.

Verified against the seeded API: dashboard courses arrive in the expected order with breakfast = planned
dish + bread + two adamas and dinner = planned dish + bread + one adama; `خبز بلدي سريع` is searchable,
has `meal_types = []`, tags `basic/bread/component/quick`, the yeast ingredient and five practical steps.
`cargo test --workspace` passes 23 tests, `python scripts/smoke_guidance.py` passes 30/30, the Svelte
build succeeds, and browser checks show the new table, quick-basics links, `?open=` modal and Arabic/
English labels with no console errors.

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
- Frontend form labels are programmatically bound and the Svelte build has no accessibility warnings;
  a full WCAG audit has not been performed.
- `JWT_SECRET` defaults to a development value; must be overridden in any deployment.

## Decision / constraint notes

- The master prompt's 19-crate layout was consolidated into 5 libraries + API + worker.
- No Tailwind; CSS variables in `apps/web/src/app.css` instead.
- All money values in JSON and DB are integer minor units (piasters). The UI divides by 100 only for display.
