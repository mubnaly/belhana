# Changelog

## 0.2.7 — 2026-11-19 (household food preference, live servings, confirmed prices, Android)

### Added
- Migration `0014_selected_diet` stores `household.selected_diet_id`; `/more/diets` now lets the
  household choose one food preference that drives every recipe list, generated plan, dashboard,
  occasion menu and AI recipe suggestion. Tayyibat and keto were researched first and are documented
  with sources and safety framing in `docs/20-DIET-SYSTEMS-EVIDENCE.md`.
- `belhana_core::diet_filter` applies explainable keyword matching to recipe names, ingredients and
  tags. Hard allergy/exclusion rules still take precedence, informational plans apply no filter, and
  a no-match state offers a way back to `/more/diets`. Diets are food preferences, not medical plans.
- Migration `0015_confirmed_prices` introduces a distinct `confirmed` review status (not the clinical
  `verified` status). Prices added or edited in the app become confirmed automatically, remain fully
  editable, and retain `previous_amount_minor` plus audit events. Ambiguous unit/date/product rows
  still need their specific confirmation before costing.
- The recipe modal now uses the household member count as its serving count. Changing it rescales
  per-serving cost and nutrition synchronously and saves the new count back to the household
  (debounced); deep links can also pass `?open=<id>&servings=<n>`.

### Changed
- Bottom navigation swaps `/inventory` for `/more/prices`; inventory remains reachable from `/more`.
- The prices page states that every entered/edited price is confirmed and editable; confirmed rows
  are no longer offered the old “confirm” action and are styled as usable, not clinical, prices.
- The weekly plan is grouped by day with meal cards in the same responsive row, and card/table labels
  wrap only between words so Arabic and English terms are never split mid-word.
- Android project generation uses `npx tauri android init --ci --skip-targets-install` on Windows so
  it does not try to install extra Rust targets; the API base differs for emulator/device use.
  `frontendDist` now points at `../../web/build` (relative to `src-tauri`) so production desktop and
  Android builds actually find the built web bundle. The Tauri shell now has a shared `belhana_desktop_lib`
  library crate with the mobile entry point, which Gradle needs to link the Android `.so`.

### Verified
- `cargo check --workspace --offline` and `cargo test --workspace --offline` pass 38 tests;
  `npm --prefix apps/web run build` is clean.
- `npx tauri android build --apk --debug -t aarch64 --ci` produced
  `apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk`.
- See `docs/17-TESTING.md` and `IMPLEMENTATION_STATUS.md` for the API/browser smoke results.

## 0.2.6 — 2026-11-19 (macros, diet & athlete guidance, theme in settings)

### Added
- Researched the nutrition/diet/sport rules online before coding; the evidence and source URLs are
  recorded in `docs/19-NUTRITION-AND-SPORT-EVIDENCE.md`.
- Migration `0013_product_macros` adds editorial `protein_per_100g`, `carbs_per_100g`,
  `fat_per_100g` and `fiber_per_100g` to `products`, seeded idempotently alongside calories.
  `belhana_core::nutrition` adds `MacroEstimate`, `macro_from_row`, `recipe_macro_map`, MET burn,
  Mifflin–St Jeor BMR, activity factors and macro targets, all with unit tests.
- Estimated macros now appear next to calories on recipes and their ingredients, plan slots,
  today's table, quick basics, shopping items and stock lots (`MacroFacts.svelte`, `≈` labels).
- Two new pages linked from `/more`: `/more/diets` (iron, copper, zinc, calcium, vitamin D, B12 and
  folate food suggestions; Mediterranean, elimination and balanced-family plans) and
  `/more/athletes` (profile-based BMR/TDEE/macros, MET exercise burn, and pre/post-workout timing
  with alternatives). Backed by `GET /api/nutrition-guide` and `POST /api/nutrition/athlete-plan`.
- Corrected the illustrative burn figures from the request: MET-based estimates for a 70 kg adult
  are ≈245 kcal/hour for brisk walking and ≈245–420 kcal/hour for weight training, not 300/900.
  The app explains the assumption instead of presenting those numbers as facts.

### Changed
- Theme selection moved from the floating app button into a dedicated card on `/more/settings`;
  the `belhana.theme` persistence and first-paint restore are unchanged.
- Added `diet`, `activity`, `flame`, `heart` and `target` monochrome SVG icons and Arabic/English
  labels for all new nutrition, diet, exercise and timing terms.

### Verified
- `cargo check --workspace` and `cargo test --workspace` pass 30 tests (7 new nutrition/guidance
  tests); `npm run build` in `apps/web` is clean.
- API smoke on a fresh database: nutrition guide 200 with 3 diets, 7 micronutrients, 10 exercises,
  4 timing sections and 10 sources; athlete plan 200 for a 70 kg adult with `bmr=1649`,
  `tdee=2556`, `protein_g=98`, `carbs_g=335.4`, `fat_g=91.4`, `burn_kcal=245` and `met=3.5`.
  Recipe, dashboard, plan-slot and shopping responses carry `estimated_macros` (13/14 shopping
  items with a catalog product match).
- Browser pass over all 18 app screens in Arabic/English and light/dark: 0 console errors, no emoji,
  no floating `.theme-toggle`, both new `/more` tiles present, Settings Light/Dark buttons switch
  and persist the theme, athlete recalculation, diet tabs/accordions/sources and language toggles
  work. The two remaining `dashed` borders are intentional `.notice` callouts, not slot underlines.

## 0.2.5 — 2026-11-19 (estimated calories, light/dark notes & monochrome icons)

### Added
- Estimated calories across the whole app. Migration `0012_product_nutrition` adds `kcal_per_100g`
  and `grams_per_unit` to `products`, seeded from editorial category defaults with named overrides
  (idempotently, like the rest of the catalog). New `belhana_core::nutrition` helpers (`grams_for`,
  `kcal_for`, `kcal_from_row`, `recipe_kcal_map`) compute calories from ingredient quantities and
  product nutrition rather than storing them as facts.
- Calories now surface everywhere they matter: recipe cards, recipe detail and ingredient rows,
  planned meal slots, today's table, quick basics, shopping items, inventory/stock lots, expiring
  stock, plan inventory usage and the dashboard. Every value is clearly marked as an estimate with
  `≈` and an “estimated” label (`kcalEstimate`); no verified nutrition, prices or stock is invented.
- A matching **light** “notes on the wall” theme alongside the dark one. Both keep the same cartoon
  sticky-note language (dotted wall, taped/tilted notes, ruled lines, sticker chips, offset shadows)
  with warm-cream-paper and charcoal/teal-plus-amber palettes respectively. Dark is not a generic
  purple. A floating toggle in the app bar persists the choice to `localStorage` (`belhana.theme`),
  restored by an early script in `app.html` to avoid a flash of the wrong theme.
- Reusable inline-SVG `Icon.svelte` set: every path draws with `currentColor`, so icons are
  single-color in both themes. All colored emoji were replaced across every screen (including login,
  register, settings, family, prices and imports) with these icons, and like/dislike preferences use
  icon buttons with proper labels.

### Changed
- Today's table is now a responsive, side-by-side grid of sticky-note cards (`auto-fit`/`minmax`)
  with tape and tilt details; the same note treatment is applied throughout the app. All note cards
  reflow cleanly from phone to desktop.
- Form controls are programmatically associated with their labels (48 labels bound across the app),
  and the `mealTypes` checkbox group uses a labelled field group. The Svelte build is now free of
  accessibility warnings.

### Fixed
- Removed the CSS `::before` estimate glyph that duplicated the `≈` already emitted by `kcalText()`;
  estimates now show a single `≈`.
- Removed the dashed underlines from plan slots.

### Verified
- API smoke check: recipes 5/5 with kcal; dashboard ordered breakfast → morning snack → lunch →
  afternoon snack → dinner; shopping 12 items (11 with kcal); inventory lot `estimated_kcal = 2040`;
  plan 6/6 slots with kcal.
- Browser checks on `/`, `/recipes`, `/plan`, `/shopping`, `/inventory` and `/more` in both themes:
  no emoji, no dashed borders, 0 bad (non-`currentColor`) icons, no console errors, estimate labels
  present, theme toggle works, note grid is responsive, and recipe-modal ingredient calories render.
- `cargo test --workspace` passes 23 tests; `npm run build` in `apps/web` is clean.

## 0.2.4 — 2026-11-19 (dark wall theme & cleaner recent prices)

### Added
- A dark, cartoon “notes on the wall” theme in `apps/web/src/app.css`: deep dotted wall background,
  slightly tilted sticky-note cards with paper tape and ruled lines, chunky outlined buttons with hard
  offset shadows, highlighter strokes under section titles, sticker chips, dark inputs, and a dark
  bottom navigation with an active-note highlight. The theme is global, so every screen inherits it.
- Native controls follow the dark scheme (`color-scheme: dark`), including scrollbars, selects and date
  pickers. Layouts straighten and transitions stop when the user prefers reduced motion.

### Fixed
- The dashboard “recent prices” list now shows one row per product (its latest observation) instead of
  letting repeated egg observations fill the whole list. Unit and review status are shown in Arabic
  (`قطعة`, `مؤكد`, …) while English keeps the raw labels.
- The assistant error bubble uses the dark theme instead of a light pink background.

### Verified
- Dashboard returns five distinct products with Arabic units/status; `cargo test --workspace` passes 23
  tests; the Svelte production build succeeds; browser checks of `/`, `/more` and the recipe modal show
  the dark wall, taped cards, sticker tiles and no console errors.

## 0.2.3 — 2026-11-19 (daily table, recipes nav & quick homemade basics)

### Added
- A dedicated Recipes slot in the bottom navigation (`/recipes`) after Plan, with compact sizing so
  six items still fit on a phone.
- Structured dashboard `today_table`, ordered breakfast → morning snack → lunch → afternoon snack →
  dinner. Today's planned dishes become the mains; each meal is completed with bread, at least two
  adamas at breakfast, rice when lunch is not already a rice dish, a green salad at lunch, bread plus
  one adama at dinner, and fruit/vegetable snacks between meals. When no planned main exists, a
  deterministic library suggestion is shown and marked «مقترح».
- `quick_basics` on the home page surfaces easy homemade bread, white rice, green salad, a side dish
  and the two snacks, each opening its recipe directly.
- `/recipes?open=<recipeId>` opens the recipe detail modal on load, so home-table dish links work in
  one tap.
- Eight quick component recipes with empty `meal_types` (so the planner never treats them as full
  meals): خبز بلدي سريع بمرة تخمير قصيرة, أرز أبيض, سلطة خضراء, جبنة بيضاء بالزيت, بيض مسلوق,
  زبادي بالخيار, سناك فواكه and سناك خضار. Added the missing `خميرة` catalog product.
- System recipe refresh now also updates `meal_types` and tags (delete-and-reinsert), idempotently, so
  existing databases receive the new `bread`, `adama`, `salad`, `rice` and snack tags.

### Verified
- Dashboard courses arrive in the order above; today's breakfast shows the planned dish plus bread and
  two adamas, and dinner shows bread plus one adama.
- `خبز بلدي سريع` is searchable, has `meal_types = []`, tags `basic/bread/component/quick`, the yeast
  ingredient and five practical steps.
- `cargo test --workspace` passes 23 tests; `python scripts/smoke_guidance.py` passes 30/30; the Svelte
  production build succeeds; browser checks show the new table, quick-basics links, `?open=` modal and
  Arabic/English labels with no console errors.

## 0.2.2 — 2026-11-18 (meal search, local saving & gram display)

### Added
- Server-side recipe search now matches Arabic and English names, the description, ingredient raw
  names, and tags. Empty or whitespace-only queries are ignored instead of filtering everything out.
  The recipes screen debounces typing, drops stale responses, and shows a result count.
- A “not found” card appears when a search has no matches. It offers an external Google search for the
  dish and opens the new-recipe form prefilled with the searched name, so the meal can be added and
  saved locally through the existing API.
- Local meal bookmarks: the star on a recipe card or in its detail modal stores the meal in
  `localStorage` (`belhana.savedRecipes`). A “saved only” filter shows bookmarked meals and survives
  reloads without server changes.
- `formatQty(value, unit)` in the web API client shows weights below 1 kg in grams
  (`0.5 kg` → `500 g`) and trims floating-point noise everywhere quantities are displayed. Stored
  values and form inputs are unchanged. Applied to recipes, shopping, inventory, dashboard expiring
  stock, plan inventory usage and batch-cooking guides.

### Verified
- Recipe search by name (`كشري` → 1 result), by ingredient (`طماطم` → 16 results), and by a nonsense
  query (0 results); the unfiltered list still returns 34 recipes.
- `cargo test --workspace` passes 23 tests; the Svelte production build succeeds.

## 0.2.1 — 2026-11-18 (recipe methods & famous Egyptian dishes)

### Added
- Full step-by-step cooking methods for all 18 existing system recipes, including soaking times, heat
  levels, doneness cues, and make-ahead/storage notes. The steps are original Belhana content written
  for this app, not copied recipe text.
- 16 famous Egyptian dishes with complete methods: رقاق باللحمة، حواوشي، مكرونة بشاميل، كفتة مشوية،
  لوبيا باللحمة، صينية بطاطس باللحمة، كبدة إسكندراني، سمك سنجاري، محشي كرومب، فتة لحمة، بسبوسة،
  أم علي، كنافة، قطايف، مهلبية، فطير مشلتت.
- Seven specialty products used by the new dishes: سميد، جوز هند، مكسرات مشكلة، كنافة، قطايف، نشا،
  فانيليا (all with valid unit codes).
- `seed_recipes` now refreshes the description, timing, cooking method, and ordered steps of existing
  system recipes on startup, so improved instructions reach existing databases without duplicates.
  Ingredient rows and costing are intentionally left untouched.

## 0.2.0 — 2026-11-17 (guidance & purchasing extensions)

### Added
- Migration `0011_guidance_and_purchasing`: `leftover_carry_over` on stock lots, `buy_by_amount_minor`
  on shopping items, and a seeded `meal_tips` table with 25 editorial tips across 9 categories.
- Four new household rule types: `ai_instruction` (a free-text sentence injected into the AI system
  prompt), `meal_staple` (preferred product per breakfast/lunch/dinner), `fixed_spend` (buy spices and
  similar items by a fixed amount of money, e.g. 20 EGP of salt), `leftover_carryover`. Six defaults
  are seeded idempotently for every new household (marked `notes = 'system-default'`).
- Manual price entry with package metadata (`package_size`, `package_unit`), so 30 eggs for 130 EGP can
  be compared per-piece with 15 eggs for 65 EGP. Cheapest-per-unit selection now compares all
  observations in one reference unit and skips rows it cannot resolve rather than guessing.
- Editing of confirmed price observations: `PUT /api/prices/{id}` keeps the prior value in
  `previous_amount_minor` and writes a `price_review_events` row with `action = 'edited'`.
- Leftover carry-over: lots default to `carry_over = 1`, appear in `GET /api/inventory/leftovers`, can be
  toggled via `POST /api/inventory/lots/{id}/carry-over`, and are consumed first in FEFO ordering.
- Meal tips page (`/more/tips`) with category filters, plus staple-fit and leftover-use explanation
  text in generated plans.
- Fixed-spend shopping lines (`unit = "spend"`, `buy_by_amount_minor`) generated from `fixed_spend`
  rules; completing a trip records the actual expense but creates no fake stock lot for them.
- `scripts/smoke_guidance.py` — 30 end-to-end checks over the new rules, tips, prices, leftovers,
  planning, shopping and audit flows (Arabic expectations kept in UTF-8 `.txt` files).

### Fixed during verification
- Price INSERT placeholder count (18 values for 19 columns) in `create_observation`.
- Cheapest-price comparison no longer mixes units; `carton` never converts to `piece` without package
  metadata.
- Smoke audit query uses the correct `price_review_events.observation_id` column.
- Added three package-price unit tests in `belhana-core::costing` (`cargo test --workspace`: 23 pass).
- Shopping quantities no longer show floating-point noise (`1.5999999999999999 kg` → `1.6 kg`).
- Bottom-nav labels now switch language with the rest of the page.
- Recipe seeding is now truly idempotent. `create_recipe` wrote `is_system = 0` while the seed guard
  checked `is_system = 1`, so every startup re-inserted all 18 system recipes (216 rows in the dev DB).
  Household-less recipes are now stored as system recipes and the guard matches `household_id IS NULL`;
  the dev DB was deduplicated after repointing plan/event references (18 recipes, plans preserved).

### Known issues
- See `IMPLEMENTATION_STATUS.md`. No live AI-provider key tested; Postgres/Tauri launch/offline sync
  remain out of scope.

## 0.1.0 — 2026-11-17 (MVP implementation)

### Added
- Rust workspace: `belhana-common`, `belhana-db`, `belhana-auth`, `belhana-core`, `belhana-ai`,
  `apps/api`, `workers/maintenance_worker`.
- 10 SQLite migrations covering auth/households, family/rules, catalog/units, merchants/prices,
  recipes, inventory, planning, shopping, occasions and AI operations.
- Idempotent seed: 17 categories, ~85 products, 18 recipes, 9 batch-prep guides, 7 user baselines,
  Olad Zain merchant + 89-line post parsed into 96 unreviewed price observations.
- 16 API route modules (~80 handlers) with JWT auth, household scoping, CORS, tracing and health check.
- Deterministic recipe costing, 7-day meal planning, consolidated shopping, purchase and inventory flows,
  occasion menus and planned-vs-actual reporting.
- Price review workflow with date/unit/product confirmation, audit log and stale marking.
- AI tool loop with read-only tools, proposal-only mutations, manual mode when no key is configured.
- SvelteKit SPA with Arabic RTL default, English switch and 15 screens.
- Tauri 2 desktop shell and maintenance worker.
- Documentation set (`README`, `PROGRESS`, `IMPLEMENTATION_STATUS`, `docs/01..18`), `.env.example`
  and a Postgres docker-compose file for future migration.

### Fixed during implementation
- Axum 0.8 path parameter syntax (`{id}`).
- SQLite REAL→i64 decoding in inventory valuation (explicit `CAST AS INTEGER`).
- `mark_prepared` household join path.
- `PlannedMealOut.id` propagation from saved plans.
- Seed data UTF-8 termination and static-lifetime aliases; Arabic smoke payloads moved to JSON files.

### Fixed during final browser/compile verification
- Missing Svelte 5 `children` prop in the root layout (caused a blank page on first load).
- Auth tokens restored before child pages mount, so a hard refresh no longer redirects to login.
- Language switch re-renders page content via a shared reactive store and `{#key lang}`.
- Added `GET /api/imports/batches/{id}/source-post` for the price-review original-post view.
- Real `Money` `Add`/`Sub` trait implementations (explicit integer piasters) instead of infinite recursion;
  removed a duplicate Arabic quantity pattern and an unused assignment so `cargo check` is warning-free.
- Generated `icons/icon.ico` so the Tauri crate's Windows resource step and `cargo check` succeed.

### Known issues
- See `IMPLEMENTATION_STATUS.md` — chiefly SQLite-only persistence today, no live AI provider test,
  no OCR/CSV/Facebook API, no offline sync, non-blocking a11y warnings in the SPA.
