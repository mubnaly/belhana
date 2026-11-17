# 18 — Deployment and operations

## Local development (current, supported)
```powershell
$env:BELHANA_DATABASE_URL = "$env:TEMP\opencode\belhana.db"
$env:BELHANA_PORT = '8080'
cargo run -p belhana-api
# second terminal
cd apps/web; npm install; npm run dev
```
Migrations and idempotent seeding run at API startup. Copy `.env.example` and adjust values.

## Web production
`cd apps/web && npm run build` writes a static SPA to `apps/web/build` (adapter-static, fallback
`index.html`). Serve it behind any static host/reverse proxy; route `/api` to the API server and set
`BELHANA_API_BASE` at build time if the API is on another origin.

## Desktop
`cd apps/desktop && npm install && npm run tauri dev` (or `npm run build` for installers). Requires
WebView2 on Windows. The API server must be running; the WebView calls `http://127.0.0.1:8080`.
The desktop Tauri crate is a standalone workspace so server builds do not need WebView toolchains.

## Maintenance worker
`BELHANA_DATABASE_URL=... cargo run -p maintenance_worker -- --once` for a single sweep; omit `--once`
for an interval loop. Jobs: mark prices stale after `BELHANA_STALE_DAYS` (default 30), create deduplicated
expiry reminders, prune notifications older than 30 days.

## Postgres migration path (not yet executed)
1. Add `sqlx` `postgres`/`runtime-tokio` features and change `belhana_db::open_pool` to accept a
   `postgres://` URL.
2. Translate the 10 SQLite migrations: `AUTOINCREMENT` → identity/sequence, `TEXT` UUID ids → `uuid`
   or keep TEXT, JSON TEXT → `jsonb` (or keep TEXT), `CAST(... AS INTEGER)` → `::bigint`.
3. Recreate migrations in `database/migrations/postgres`, run against the docker-compose Postgres,
   verify with a fresh seed and the smoke scripts.
4. Replace the any-origin CORS policy, set strong `BELHANA_JWT_SECRET`, terminate TLS at the proxy.

## Backup/restore (operator responsibility today)
Copy the SQLite file while the API is stopped, or use `sqlite3 .backup`; restore by replacing the file
and restarting. Automated backup/restore tests are still open.
