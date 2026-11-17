# 02 — Architecture

## Layers

```
SvelteKit SPA (apps/web)  ──/api──>  Axum API (apps/api)
Tauri WebView (apps/desktop) ──HTTP──┘        │
                                              ├─ belhana-core (deterministic domain logic)
                                              ├─ belhana-ai  (provider abstraction + tool loop)
                                              ├─ belhana-auth (Argon2 + JWT)
                                              └─ belhana-db  (SQLx pool + migrations) ──> SQLite
                                    maintenance_worker ─────────────────────────────> same DB
```

## Crate consolidation
The master prompt sketched ~19 crates. This implementation uses one crate per layer:
`common` (money, units, Arabic text, errors), `db` (pool/migrations), `auth`, `core`
(household/family/pricing/imports/recipes/inventory/planning/shopping/occasions/dashboard),
`ai`, `api` (16 route modules), `maintenance_worker`. Feature boundaries remain explicit at module level.

## Request flow
1. Client sends `Authorization: Bearer <access_token>`; `AuthUser` extractor validates JWT and loads
   `user_id`/`household_id`.
2. Route deserializes input, calls a `belhana_core` function with the `SqlitePool` and household id.
3. Core writes/reads via SQLx; money columns are INTEGER piasters.
4. Errors map `CommonError` → `ApiError` → JSON `{ "error": { "message": ... } }`.
5. Frontend retries once with the refresh token on 401, then logs out.

## Cross-cutting choices
- Startup: migrate → seed (idempotent) → bind `BELHANA_HOST:BELHANA_PORT`; health check is `GET /healthz`.
- CORS allows all origins (local/dev convenience; lock down for production).
- Tracing via `RUST_LOG` with default `info,sqlx=warn`.
- AI is optional: without a provider key the API returns a manual-mode message; mutating tools only
  create `ai_action_proposals` rows.
- Tauri shell hosts the same SPA build; `api.js` detects `tauri:` protocol and targets `127.0.0.1:8080`.
