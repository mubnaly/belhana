# Changelog

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
