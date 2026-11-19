# 17 — Testing

## Automated
- `cargo test --workspace --offline` → 38 tests pass:
  - `belhana-common` (8): money arithmetic/formatting, unit parsing/conversion, Arabic text helpers.
  - `belhana-auth` (2): password hashing and JWT issue/verify/refresh.
  - `belhana-core` (28): planning scenarios, rules, costing, date/parse behaviour, macro/nutrition
    estimates, guidance helpers, diet keyword filtering, the distinct `confirmed` price status, and
    keeping unresolved units flagged after edits.
- `cargo check --workspace --offline` → compiles cleanly with no warnings.
- `npm --prefix apps/web run build` → SvelteKit static SPA builds to `apps/web/build` cleanly.
- `cargo check` in `apps/desktop/src-tauri` → Tauri 2 shell checks cleanly (icon.ico generated).
- Android (Windows host, complete NDK 26.1.10909125 + JDK 17 + `aarch64-linux-android` Rust target):
  `cd apps/desktop && npx tauri android build --apk --debug -t aarch64 --ci` → built
  `gen/android/app/build/outputs/apk/universal/debug/app-universal-debug.apk` (125.46 MB); Kotlin
  deprecation/Java 8 source-target warnings only, no build errors. The APK was not launched on a device.
- On this Windows host always use `--offline` for Cargo: a plain `cargo check` tried to refresh the
  crates.io index and failed with `memory allocation of 760 bytes failed`.

## API smoke — diet & confirmed prices (fresh DB, 2026-11-19)
Ran the rebuilt API from a separate target directory on port 8081 against a throwaway SQLite file:
- `GET /healthz` and login with the seeded `nutrition@belhana.app` account succeeded.
- `GET /api/household` started with no `selected_diet_id`; `PUT /api/household` set
  `selected_diet_id="tayyibat"` and `household_size=3`.
- `GET /api/recipes` returned 42 recipes before the preference and 3 matching recipes after it.
- `POST /api/meal-plans` returned `selected_diet_id="tayyibat"`, a Tayyibat `diet_note.ar` and 4 slots
  across the two requested days.
- `POST /api/prices` returned `review_status="confirmed"`; `PUT /api/prices/{id}` kept `confirmed`,
  set `previous_amount_minor=1250` and `amount_minor=1300`; the confirmed-filtered list contained the
  observation. No `verified` status was asserted.

## Manual smoke scripts (`scripts/`)
Run the API against a throwaway SQLite file, then execute the PowerShell scripts. Arabic payloads are
kept in `.json` files because PowerShell can misread BOM-less UTF-8 scripts. Remember to assign the
token to a variable (`$tok = $reg.access_token`) — PowerShell does not expand object properties inside
a double-quoted string.

Exercised flows: register/login/refresh rotation; household/members/rules/budget status/what-if;
household diet selection and filtered recipes/plans; meal-plan generate/save/read; shopping
generation/complete; inventory add-lot/valuation; price review (`confirm_unit`) plus automatic
`confirmed` status on create/edit; mark-prepared; occasion menus/approve/shopping/actual-vs-planned;
recipe cost with unknown prices; batch guides; merchants; import parse preview; AI manual mode.

## Browser E2E (2026-11-17)
With the seeded API and `vite dev` running, a headless browser exercised login → dashboard → bottom
navigation; refreshed the dashboard and confirmed the session persists; visited `/plan`, `/shopping`,
`/inventory`, `/recipes`, `/more` and all nine `/more` subpages; opened an import batch and reviewed its
original post and observations; and toggled Arabic/English (including document direction). No console
errors remained. This pass found and fixed the Svelte 5 `children` prop, auth-restore ordering and
language-reactivity issues described in `CHANGELOG.md`.

### Household preference / prices pass (2026-11-19)
With the rebuilt API (migrations `0014_selected_diet` + `0015_confirmed_prices`) and `vite dev` on
`127.0.0.1:5173`, the browser set Tayyibat on `/more/diets` and confirmed the choice persisted to the
household record. `/recipes` then showed the selected-preference banner and only the matching meals;
opening a meal changed servings 1 → 3 and immediately updated total kcal ≈709 → ≈2127 and ingredient
quantities (rice 125 g → 375 g). `/plan` showed the Tayyibat diet note and day groups with three meal
notes on the same row. The bottom nav showed «الأسعار» and no «المخزن». `/more/prices` showed the
confirmed-but-editable hint and distinct «مؤكد» / «موثّق» filters; a created then edited price remained
`confirmed` with `previous_amount_minor` preserved in the API smoke. Arabic RTL and English LTR
toggles and the Settings-only Light/Dark switch were checked on the affected screens with 0 console
errors. Meal-card computed styles keep `word-break: normal` / `overflow-wrap: normal` so words are not
split mid-word.

## Master prompt scenarios A–H
| Scenario | Coverage |
| --- | --- |
| A Weekly plan | Plan generator + shopping smoke flow |
| B Preferences | Member preferences/exclusions and rule tests |
| C Conflicting price data | Review workflow + best-price selection tests |
| D Unresolved units | `needs_unit_confirmation` rows stay unreviewed; `confirm_unit` audit |
| E Date inconsistency | Parsed weekday mismatch flagged; `confirm-date` API |
| F Shopping and stock | Purchase → inventory lot → mark-prepared smoke flow |
| G Special occasion | Menus/approve/shopping/actual-cost smoke flow |
| H Multiple devices | Same JWT/household API; two clients not manually tested |

## Not tested
Live Gemini/OpenAI responses, native Tauri launch on a machine without WebView2, Postgres port,
launching the Android APK on a device (build only), iOS, OCR/CSV imports, load/performance and a
formal security audit.
