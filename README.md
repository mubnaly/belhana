# بالهنا — Belhana

Arabic-first household food budgeting, meal planning and price-intelligence app for Egyptian families.
The guiding rules are **deterministic calculations, traceable prices, visible uncertainty, reviewable AI actions**.

> Current delivery: working Rust/Axum API, SvelteKit SPA and Tauri 2 desktop shell, backed by SQLite
> for local development. The schema is written to be Postgres-compatible; a Postgres deployment path
> is documented in [docs/18-DEPLOYMENT.md](docs/18-DEPLOYMENT.md). See
> [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for exactly what is and is not implemented.

## Quick start (local development)

Requirements: Rust 1.77+ (tested on 1.99), Node.js 20+ and npm. No database server is required —
SQLx uses a local SQLite file and applies migrations at startup.

```powershell
# 1. API (terminal 1) — http://127.0.0.1:8080
$env:BELHANA_DATABASE_URL = "$env:TEMP\opencode\belhana.db"
$env:BELHANA_PORT = '8080'
cargo run -p belhana-api

# 2. Web UI (terminal 2) — http://127.0.0.1:5173, proxies /api to 8080
cd apps/web
npm install
npm run dev

# 3. Optional maintenance worker (stale prices, expiry reminders, notification cleanup)
$env:BELHANA_DATABASE_URL = "$env:TEMP\opencode\belhana.db"
cargo run -p maintenance_worker -- --once
```

Health check: `GET http://127.0.0.1:8080/healthz` → `{"app":"belhana-api","ok":true}`.
On first start the database is migrated and seeded (categories, products, recipes, batch-prep guides,
user baselines and the Olad Zain post as an **unreviewed** import batch).

Production web bundle: `cd apps/web && npm run build` → static SPA in `apps/web/build`.
Desktop shell: `cd apps/desktop && npm install && npm run tauri dev` (WebView2 on Windows; API must be running).

## Repository layout

| Path | Purpose |
| --- | --- |
| `crates/belhana-common` | Money (integer piasters), units, Arabic text/time helpers, errors |
| `crates/belhana-db` | SQLite pool, migrations, shared DB errors |
| `crates/belhana-auth` | Argon2 password hashing, JWT access/refresh tokens |
| `crates/belhana-core` | Domain logic: household, family, prices, imports, recipes, inventory, planning, shopping, occasions, AI tools |
| `crates/belhana-ai` | Gemini/OpenAI provider abstraction, tool schemas, proposal guardrails |
| `apps/api` | Axum HTTP server, 16 route modules, seeding, CORS, tracing |
| `apps/web` | SvelteKit 5 + TypeScript-ready SPA, Arabic RTL default, English switch |
| `apps/desktop` | Tauri 2 shell hosting the SPA |
| `workers/maintenance_worker` | Stale-price sweep, expiry notifications, pruning |
| `database/migrations/sqlite` | 10 ordered SQLx migrations |
| `scripts` | PowerShell smoke tests (Arabic payloads kept in `.json` files) |
| `docs` | Product, architecture, schema and operational documentation |

The original 19-crate sketch in the master prompt was consolidated into six crates: one per
architectural layer instead of one per feature. See `docs/02-ARCHITECTURE.md`.

## Tests

```powershell
cargo test --workspace
cd apps/web && npm run build
```

20 unit/integration tests cover money/units/text, auth tokens, and planning/scenario logic.
`scripts/smoke*.ps1` exercise the full API against the seeded SQLite database: auth + refresh rotation,
budget, plan generation, shopping/purchase/inventory, price review, occasions, imports and AI manual mode.

## Safety properties

- Money is `i64` **piasters** end-to-end; conversion to EGP happens only at display time.
- Price observations are dated evidence with `review_status`; ambiguous rows stay out of verified calculations.
- User baselines are stored separately from merchant quotes and never overwrite them.
- The meal planner, shopping list, recipe costing and occasion costing are deterministic.
- AI can call read-only tools; every mutating tool creates an `ai_action_proposals` row for human approval.
- All domain queries are scoped to `household_id`; JWTs carry the household claim.

See [PROGRESS.md](PROGRESS.md) for the phase-by-phase build log and
[docs/01-PRODUCT-SPECIFICATION.md](docs/01-PRODUCT-SPECIFICATION.md) for the product contract.
