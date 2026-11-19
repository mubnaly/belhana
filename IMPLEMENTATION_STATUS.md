# Implementation status

Last updated: 2026-11-19. This file reports the code that exists and the checks that were run;
it deliberately does not claim unverified integrations.

## Build state

| Component | State | Evidence |
| --- | --- | --- |
| `belhana-common` | ✅ | 8 unit tests; money is integer piasters |
| `belhana-db` | ✅ | pool + SQLx migrations (15 files, incl. `0011`–`0015`) |
| `belhana-auth` | ✅ | 2 tests; Argon2 + JWT access/refresh rotation |
| `belhana-core` | ✅ | 28 tests; domains implemented against SQLite (incl. package-price, macro, MET/BMR, guidance, diet-filter and confirmed-price tests) |
| `belhana-ai` | ✅ | provider abstraction, tool schemas, proposal guardrails; instruction rules in system prompt; no live API key tested |
| `apps/api` | ✅ | route modules (incl. nutrition guide + athlete plan); startup migrate + seed; healthz |
| `apps/web` | ✅ | Arabic RTL/English SPA, theme in Settings, light/dark sticky-note themes, monochrome SVG icons, backend-free demo client, static build clean + browser E2E pass on `/belhana/` demo |
| `apps/desktop` | 🟡 | Tauri 2 shell compiles (`cargo check`); WebView2 required to launch the Windows binary |
| `apps/desktop` (Android) | 🟡 | app ID `com.dawwar.online.allama.belhana`, logo/splash/signing config updated; release APK rebuild started in-session (verification pending in this snapshot) |
| `workers/maintenance_worker` | ✅ | `--once` sweep ran clean (stale prices, expiry reminders, pruning) |
| `.github/workflows` | ✅ | Rust tests, web build, GitHub Pages deploy (`BELHANA_BASE=/belhana`), Android APK artifact workflows added |

## MVP checklist

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
26. ✅ Household-wide food preference stored on `households.selected_diet_id`; recipes, plans,
    dashboard, occasions and AI recipe tools follow it. Tayyibat and keto added from researched
    evidence with explicit “not a treatment plan” framing.
27. ✅ Recipe modal serves the household member count; changing it rescales cost, kcal and ingredient
    quantities immediately and saves the count back to the household.
28. ✅ Prices entered or edited in the app are `confirmed` (not clinical `verified`), remain editable,
    and keep `previous_amount_minor` plus review events; ambiguous import rows still need unit/date/
    product confirmation.
29. ✅ Backend-free static demo: bundled seeded snapshot served in-browser on static hosts (GitHub Pages),
    with demo/API mode controls, `BELHANA_BASE=/belhana` build, SPA `404.html` fallback and `.nojekyll`.
30. 🟡 Android release APK rebuild after logo/splash/app-ID/signing updates: in progress/pending verification
    (previous verified release APK was 5.57 MB, debug-key signed). Google Play upload requires an owner-provided
    upload keystore and a signed release AAB.

## Backend-free deployment, logo/splash & Play readiness (2026-11-19)

The web client now includes a backend-free demo path (`apps/web/src/lib/demo.js`, `demo-data.json`) captured by
`scripts/capture_demo_fixture.py` from a seeded API. Demo mode auto-enables on non-localhost static hosts without
a configured API base, with explicit override via `belhana.demoMode`; localhost dev, Tauri, or configured
`belhana.apiBase` / `BELHANA_API_BASE` keep using the real network client. GitHub Pages builds use
`BELHANA_BASE=/belhana`, base-aware paths, copied `404.html`, and `.nojekyll`.

Branding assets were regenerated via `scripts/generate_logo.py` and applied to the web app (logo, favicon, boot
splash, branded screens), Tauri icons, and Android mipmaps/splash resources. The Android/Tauri identifier is now
`com.dawwar.online.allama.belhana`, with Kotlin sources moved to `com/dawwar/online/allama/belhana` and optional
Play-upload signing driven by `keystore.properties` (debug-key fallback when absent). CI workflows were added for
Rust tests, web build, Pages deploy, and Android APK artifact generation.

Verified in-session: `npm --prefix apps/web run build` succeeded (with and for Pages base context as noted in
changelog/context), and browser checks against the built `/belhana/` demo showed login/dashboard/recipes/plan/
more/prices/more/settings rendering seeded content with 0 console errors in checked flows. Android release APK
rebuild/`apksigner` verification after these changes was started but not completed within this snapshot.

## Household preference, live servings, confirmed prices & Android (2026-11-19)

Research for Tayyibat and keto was recorded in `docs/20-DIET-SYSTEMS-EVIDENCE.md` before coding.
Migration `0014_selected_diet` adds `selected_diet_id` to `households`; `UpdateHousehold` and
`selected_diet_id()` expose it to the API. `belhana_core::diet_filter` performs explainable keyword
matching over recipe names, ingredients and tags. It is used by recipe listing, plan generation,
dashboard, occasion menus and AI recipe tools; allergy/exclusion rules still win, informational plans
apply no filter, and the UI has an explicit no-match state with a link back to `/more/diets`.

The recipe modal derives all numbers from `servings / recipe.default_servings`, so kcal, macros and
ingredient quantities change synchronously without mutating the immutable recipe detail. Only the
debounced write back to `/api/household` touches the network. Deep links can open a recipe with
`?open=<id>&servings=<n>`.

Migration `0015_confirmed_prices` adds the `confirmed` review status, a user-entered price state that
is usable in costing but deliberately distinct from `verified`. Create/edit paths set `confirmed`,
preserve `previous_amount_minor` and record audit events; unresolved units still produce
`needs_unit_confirmation`. The bottom nav now opens `/more/prices` instead of `/inventory`.

Verified: `cargo check --workspace --offline` and `cargo test --workspace --offline` (38 tests) clean;
`npm --prefix apps/web run build` clean. Fresh-DB API smoke: household diet null → `tayyibat`, recipes
42 → 3, plan `diet_note` present with 4 slots, created/edited price `confirmed` with
`previous_amount_minor=1250`. Browser pass in Arabic RTL and English LTR plus light/dark on
`/more/diets`, `/recipes`, `/plan` and `/more/prices`: 0 console errors, bottom nav shows الأسعار and
no المخزن, servings 1 → 3 changed ≈709 → ≈2127 kcal and rice 125 g → 375 g, plan day cards share a
row with intact words, and the new «مؤكد» / «موثّق» filters are distinct.

Android project generation with `npx tauri android init --ci --skip-targets-install` succeeded. The
first successful release build (prior to this session's branding/signing updates) produced
`apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`
(5.57 MB, debug-key signed, `apksigner verify` passed). The generated project keeps
`usesCleartextTraffic=true` in release (local/LAN HTTP API) and contains manual edits; avoid
`tauri android init`.

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

Verified: `cargo check --workspace --offline` and `cargo test --workspace --offline` (38 tests in the
current suite) clean; `npm run build` clean. API smoke on a fresh DB: guide 200 (3 diets, 7 micronutrients, 10 exercises, 4 timing
sections, 10 sources); athlete plan for 70 kg male 200 with `bmr=1649`, `tdee=2556`, `protein_g=98`,
`carbs_g=335.4`, `fat_g=91.4`, `burn_kcal=245`, `met=3.5`; 70 kg examples corrected to ≈245 kcal/h
brisk walking and ≈245–420 kcal/h lifting, not 300/900. Recipes, dashboard courses, plan slots
(3/3) and shopping items (13/14 with a product match) all returned `estimated_macros`. Browser pass
on all 18 screens in Arabic/English and light/dark: both `/more` tiles visible, 0 console errors,
no emoji, no `.theme-toggle`, `currentColor` icons, no slot underlines (the two `dashed` elements are
intentional `.notice` callout borders), athlete recalculation on 80 kg (`bmr=1749`, `burn=280`),
diet tabs/accordions/sources, language toggles, and Settings theme switching with persistence.

## Estimated calories, light/dark notes & monochrome icons (2026-11-19)

Migration `0012_product_nutrition` adds `kcal_per_100g` and `grams_per_unit` to `products`, seeded from
category defaults with named overrides in an idempotent refresh. `belhana_core::nutrition`
derives calories from ingredient quantities and product nutrition rather than storing them as verified facts;
UI surfaces prefix every value with `≈` and an “estimated” label so no invented nutrition is presented as measured.

The web ships **two** matching cartoon “notes on the wall” themes — a warm-cream light palette and
a charcoal/teal-plus-amber dark palette. Both keep dotted wall, taped/tilted notes, ruled lines, sticker chips
and offset shadows. Theme choice persists as `belhana.theme` and is restored early to avoid flash. `Icon.svelte`
provides single-color inline SVGs (`currentColor`) replacing colored emoji across screens.

Verified: API smoke and browser checks across key screens in both themes (no emoji, 0 bad icons, no console errors,
estimate labels present, responsive note grid). `cargo test --workspace` passed 23 tests at that point and the
Svelte production build was clean.

## Daily table, recipes nav and quick basics (2026-11-19)

`/api/dashboard` returns `today_table` (breakfast → morning snack → lunch → afternoon snack → dinner) with planned
or suggested mains and component sides/snacks. Recipes are in the bottom navigation and `/recipes?open=<id>` opens
the detail modal directly. Eight quick component recipes were added with empty `meal_types` and the `خميرة`
product was added to the catalog.

## Meal search and local saving (2026-11-18)

`GET /api/recipes?search=` searches names, description, ingredient raw names and tags. The UI debounces requests,
offers external-search/add-local fallback for misses, supports local star bookmarks (`belhana.savedRecipes`) and a
saved-only filter. `formatQty` renders sub-kilogram weights in grams and trims floating-point noise.

## Recipe content expansion (2026-11-18)

The shared recipe library expanded to 34 dishes with full original step-by-step methods; system recipes refresh
description/timing/method/steps in place on startup while ingredients/costing remain untouched.

## Guidance-feature verification pass (2026-11-17)

`scripts/smoke_guidance.py` passed 30/30 checks covering rules, tips, carry-over, package prices, price edits,
staple/leftover explanations, fixed-spend shopping and audit events. Recipe seeding idempotency was fixed.

## Final verification pass (2026-11-17)

Browser E2E against dev server and seeded API verified core flows and Arabic/English toggling; several frontend/auth
and Tauri compile issues were fixed (`children` prop, auth restore, language re-key, source-post endpoint, Money ops, icon).

## Known limitations / open work

- SQLite is the active database. Migrations use portable SQL, but a Postgres connection pool and a
  migration verification pass are still required before production deployment.
- No live Gemini/OpenAI key was available; provider response shapes should be smoke-tested with a real key.
- Facebook/Meta API, OCR import, CSV/Excel import, iOS builds and offline sync are not built.
- A full WCAG audit has not been performed.
- `JWT_SECRET` defaults to a development value; must be overridden in any deployment.
- Google Play distribution requires an owner-provided upload keystore and signed AAB; CI/local fallback signing is
  debug-key only.

## Decision / constraint notes

- The original 19-crate layout was consolidated into 5 libraries + API + worker.
- No Tailwind; CSS variables in `apps/web/src/app.css` instead.
- All money values in JSON and DB are integer minor units (piasters). The UI divides by 100 only for display.
- The generated Android project under `apps/desktop/src-tauri/gen/android` is intentionally maintained with manual
  edits; avoid regenerating it blindly.
