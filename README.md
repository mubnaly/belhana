# بالهنا — Belhana

Arabic-first household food budgeting, meal planning and price-intelligence app for Egyptian families.
The guiding rules are **deterministic calculations, traceable prices, visible uncertainty, reviewable AI actions**.

> Current delivery: working Rust/Axum API, SvelteKit SPA and Tauri 2 desktop/Android shell, backed by SQLite
> for local development. The web app can also run as a **backend-free static demo** (GitHub Pages) using a
> bundled seeded snapshot. The schema is written to be Postgres-compatible; a Postgres deployment path
> is documented in [docs/18-DEPLOYMENT.md](docs/18-DEPLOYMENT.md). See
> [IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for exactly what is and is not implemented.

## Highlights

- **Backend-free static demo:** on a non-localhost static host with no configured API base, the SPA auto-enables
  demo mode and serves data from `apps/web/src/lib/demo-data.json` in the browser (`localStorage.belhana.demoState`).
  Tauri, localhost dev, or an explicit `belhana.apiBase` / `BELHANA_API_BASE` use the real API. Override in Settings
  or via `localStorage.belhana.demoMode` (`1` demo, `0` API).
- **Branding:** a hand-built bowl-and-grain logo is used in the web app, as the boot splash, and across Tauri/Android
  icons and the Android splash screen (`scripts/generate_logo.py` regenerates assets).
- **Mobile identity:** Android/Tauri app ID is `com.dawwar.online.allama.belhana`, ready for a Play upload once built
  with an owner-provided upload keystore/AAB.

## Quick start (local development with API)

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

## Static web build & GitHub Pages demo

```powershell
cd apps/web
npm install
npm run build                              # static SPA in apps/web/build
$env:BELHANA_BASE='/belhana'; npm run build # GitHub Pages project-site base
```

The build writes a static SPA to `apps/web/build` (adapter-static, fallback `index.html`), copies
`404.html` for SPA deep links, and includes `.nojekyll`. For GitHub Pages project sites set
`BELHANA_BASE=/belhana` so routes/assets resolve under `/belhana/`.

The committed demo snapshot (`apps/web/src/lib/demo-data.json`) is captured from a seeded local API by
`scripts/capture_demo_fixture.py` (`demo@belhana.app` / `demo123` in the capture script). Regenerate it
after seed or demo-relevant API changes while `belhana-api` is running on `127.0.0.1:8080`.

CI/workflows in `.github/workflows/`: Rust tests, web build, GitHub Pages deploy (`BELHANA_BASE=/belhana`),
and an Android APK artifact build.

## Desktop shell

```powershell
cd apps/desktop
npm install
npm run tauri dev
```

Requires WebView2 on Windows. In dev the shell loads the Vite server; production builds bundle
`apps/web/build` (`frontendDist: ../../web/build`). The API must be running unless you use demo mode on a
static web build.

## Android app (Google Play–ready build path)

Requirements (Windows host example): Android SDK + a **complete** NDK + JDK 17 + `aarch64-linux-android` Rust target.

```powershell
$env:ANDROID_HOME='D:\programs\android_sdk'
$env:ANDROID_NDK_HOME='D:\programs\android_sdk\ndk\26.1.10909125'
$env:JAVA_HOME='C:\Program Files\Eclipse Adoptium\jdk-17'

cd apps/desktop
npx tauri android build --apk -t aarch64 --ci   # release is the default
```

**Always build release, not debug.** Release uses `strip`, `lto`, `opt-level = "s"` and `panic = "abort"`
(`src-tauri/Cargo.toml`) and Gradle minification, producing a small installable APK. The generated Android
project lives in `apps/desktop/src-tauri/gen/android` and contains manual edits (cleartext traffic for local/LAN
HTTP API, release/signing config, splash resources, moved package) — **do not rerun `tauri android init` unless you
are prepared to reapply them**.

### App identity & signing

- **Application ID / namespace:** `com.dawwar.online.allama.belhana` (`tauri.conf.json` identifier and Android Gradle config).
- **Release signing:** `gen/android/app/build.gradle.kts` reads optional `gen/android/keystore.properties` (see
  `keystore.properties.example`). If no upload keystore is provided, release falls back to the local debug key so
  builds remain installable; this is **not** suitable for Google Play upload.
- **Google Play:** create/provide an upload keystore, copy `keystore.properties.example` to
  `gen/android/keystore.properties` (gitignored) with the real values, then build a signed release **AAB** for Play
  (for example `npx tauri android build --aab -t aarch64 --ci`, depending on your Tauri/Gradle flow) and upload it
  to the Play Console. APK artifacts from CI/local builds are useful for sideload/testing.

### Output & runtime notes

- APK output: `apps/desktop/src-tauri/gen/android/app/build/outputs/apk/universal/release/app-universal-release.apk`.
- The Tauri crate includes a `belhana_desktop_lib` cdylib/staticlib target with the mobile entry point for Gradle.
- The app uses `10.0.2.2:8080` in the Android emulator; on a real phone set **Settings → Server address** to the
  API's LAN address. Cleartext HTTP is allowed in release because the app may target a local/LAN API.
- iOS requires macOS/Xcode.

## Repository layout

| Path | Purpose |
| --- | --- |
| `crates/belhana-common` | Money (integer piasters), units, Arabic text/time helpers, errors |
| `crates/belhana-db` | SQLite pool, migrations, shared DB errors |
| `crates/belhana-auth` | Argon2 password hashing, JWT access/refresh tokens |
| `crates/belhana-core` | Domain logic: household, family, prices, imports, recipes, inventory, planning, shopping, occasions, AI tools |
| `crates/belhana-ai` | Gemini/OpenAI provider abstraction, tool schemas, proposal guardrails |
| `apps/api` | Axum HTTP server, route modules, seeding, CORS, tracing |
| `apps/web` | SvelteKit 5 SPA, Arabic RTL default, English switch, backend-free demo client, static build |
| `apps/desktop` | Tauri 2 shell hosting the SPA / Android project (generated, manually maintained) |
| `workers/maintenance_worker` | Stale-price sweep, expiry notifications, pruning |
| `database/migrations/sqlite` | Ordered SQLx migrations |
| `scripts` | Smoke tests, demo fixture capture, logo asset generation |
| `.github/workflows` | Rust tests, web build, GitHub Pages deploy, Android APK artifact |
| `docs` | Product, architecture, schema and operational documentation |

## Guidance & purchasing features

On top of the MVP, each household gets seeded rules: an `ai_instruction` sentence (injected into the
AI system prompt only), `meal_staple` preferences for breakfast/lunch/dinner, a `fixed_spend` amount for
items such as spices, and `leftover_carryover`. Unused food carries into the next week via `carry_over`
stock lots, which the planner consumes first. Prices can be entered manually with package metadata and
edited afterwards with a full audit trail; prices entered/edited in the app are stored as **confirmed**
(user-entered, usable in costing, editable) and remain distinct from clinical `verified` evidence.
Cheapest-per-unit selection compares packs in one reference unit and never mixes units. The `/more/tips`
page offers filterable meal tips, and generated plans explain staple fit and leftover use. Mutating AI
actions remain proposals that require human approval.

## Tests

```powershell
cargo test --workspace
cd apps/web && npm run build
python scripts/smoke_guidance.py   # requires the API running on 127.0.0.1:8080
```

## Safety properties

- Money is `i64` **piasters** end-to-end; conversion to EGP happens only at display time.
- Price observations are dated evidence with `review_status`; ambiguous rows stay out of verified calculations.
- User baselines are stored separately from merchant quotes and never overwrite them.
- The meal planner, shopping list, recipe costing and occasion costing are deterministic.
- AI can call read-only tools; every mutating tool creates an `ai_action_proposals` row for human approval.
- All domain queries are scoped to `household_id`; JWTs carry the household claim.

See [CHANGELOG.md](CHANGELOG.md) for the release history,
[IMPLEMENTATION_STATUS.md](IMPLEMENTATION_STATUS.md) for the current build state, and
[docs/01-PRODUCT-SPECIFICATION.md](docs/01-PRODUCT-SPECIFICATION.md) for the product contract.
