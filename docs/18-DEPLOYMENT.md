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
Health check: `GET /healthz`.

## Backend-free static demo (GitHub Pages / any static host)

The web SPA can run without a backend by serving a bundled seeded snapshot in the browser.

- **How it works:** `apps/web/src/lib/demo.js` + `apps/web/src/lib/demo-data.json` satisfy API calls locally and
  persist demo state in `localStorage.belhana.demoState`.
- **Auto-detection:** demo mode auto-enables on non-localhost static hosts when no API base is configured, unless
  overridden by `localStorage.belhana.demoMode` (`1` = demo, `0` = API). Localhost dev, Tauri, or an explicit
  `belhana.apiBase` / `BELHANA_API_BASE` keep using the real network client.
- **Build for GitHub Pages (project site `/belhana/`):**
  ```powershell
  cd apps/web
  npm ci
  $env:BELHANA_BASE='/belhana'
  npm run build
  ```
  Output is `apps/web/build` with `index.html`, copied `404.html` for SPA deep links
  (`scripts/copy-pages-fallback.mjs`), and `.nojekyll`.
- **Fixture capture (dev only):** with a seeded API running on `127.0.0.1:8080`,
  `python scripts/capture_demo_fixture.py` writes `apps/web/src/lib/demo-data.json`.

## GitHub Actions & Pages

Workflows in `.github/workflows/`:

- `rust-tests.yml` — `cargo test --workspace` on push/PR to `master`.
- `web-build.yml` — `npm ci && npm run build` in `apps/web` on push/PR.
- `pages.yml` — builds with `BELHANA_BASE=/belhana` and deploys `apps/web/build` to GitHub Pages on push to
  `master` / manual dispatch.
- `android-apk.yml` — builds the `aarch64` release APK and uploads it as a workflow artifact.

To publish: push `master` to `https://github.com/mubnaly/belhana`, ensure GitHub Actions are enabled, and set the
repository Pages source to **GitHub Actions** (the deploy workflow uses `actions/deploy-pages`). The project-site
URL will be `https://mubnaly.github.io/belhana/`.

## Web production (API-backed)

`cd apps/web && npm run build` writes a static SPA to `apps/web/build` (adapter-static, fallback `index.html`).
Serve it behind any static host/reverse proxy; route `/api` to the API server and set `BELHANA_API_BASE` at build
time (or let users set the server address in Settings / `belhana.apiBase`) if the API is on another origin.

## Desktop

`cd apps/desktop && npm install && npm run tauri dev` (or `npm run build` for installers). Requires WebView2 on
Windows. `tauri.conf.json` uses `frontendDist: ../../web/build` for production bundles. The app can talk to a local
API or be used with demo mode depending on hosting/context.

## Android & Google Play

### Build requirements

- Android SDK, a complete NDK (verified: `26.1.10909125`), JDK 17, and Rust target `aarch64-linux-android`.
- Environment (Windows example): `ANDROID_HOME`, `ANDROID_NDK_HOME`, `JAVA_HOME`.

### Build commands

```powershell
cd apps/desktop
npx tauri android build --apk -t aarch64 --ci
# For Play upload, build a signed AAB with an upload keystore configured:
# npx tauri android build --aab -t aarch64 --ci
```

Release builds apply Cargo profile optimizations (`strip`, `lto`, `opt-level = "s"`, `panic = "abort"`) and
Gradle minification. **Do not ship debug APKs.**

### App ID and signing

- **App ID:** `com.dawwar.online.allama.belhana` (Tauri identifier, Android namespace/applicationId, moved Kotlin package).
- **Generated project:** `apps/desktop/src-tauri/gen/android` has manual edits (cleartext traffic enabled in release
  for local/LAN HTTP API access, signing config, splash/logo resources, package move). **Do not rerun
  `tauri android init`** unless you are prepared to reapply these changes.
- **Keystore:** copy `gen/android/keystore.properties.example` to `gen/android/keystore.properties` (gitignored)
  and set `storeFile` (relative to the Android project root), `storePassword`, `keyAlias`, `keyPassword`.
  - With a valid keystore: release is signed for upload/distribution.
  - Without it: release falls back to the local debug key so builds remain installable for testing (**not** for Play).
- **Google Play:** create an upload key/keystore owned by the publisher, configure `keystore.properties`, produce a
  signed release **AAB**, and upload via the Play Console. Enroll in Play App Signing as appropriate.

### Runtime notes

- Emulator API host: `10.0.2.2:8080`. On a physical device set **Settings → Server address** to the API's LAN address.
- Logo/splash assets can be regenerated with `python scripts/generate_logo.py` (requires Pillow).

## Maintenance worker

`BELHANA_DATABASE_URL=... cargo run -p maintenance_worker -- --once` for a single sweep; omit `--once`
for an interval loop. Jobs: mark prices stale after `BELHANA_STALE_DAYS` (default 30), create deduplicated
expiry reminders, prune notifications older than 30 days.

## Postgres migration path (not yet executed)

1. Add `sqlx` `postgres`/`runtime-tokio` features and change `belhana_db::open_pool` to accept a
   `postgres://` URL.
2. Translate SQLite migrations: `AUTOINCREMENT` → identity/sequence, JSON TEXT → `jsonb` (or keep TEXT),
   `CAST(... AS INTEGER)` → `::bigint`, etc.
3. Recreate migrations in `database/migrations/postgres`, run against docker-compose Postgres,
   verify with a fresh seed and smoke scripts.
4. Replace any-origin CORS policy, set strong `BELHANA_JWT_SECRET`, terminate TLS at the proxy.

## Backup/restore (operator responsibility today)

Copy the SQLite file while the API is stopped, or use `sqlite3 .backup`; restore by replacing the file
and restarting. Automated backup/restore tests are still open.
