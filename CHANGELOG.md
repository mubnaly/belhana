# Changelog

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
