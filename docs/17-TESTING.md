# 17 — Testing

## Automated
- `cargo test --workspace` → 20 tests pass:
  - `belhana-common`: money arithmetic/formatting, unit parsing/conversion, Arabic text helpers.
  - `belhana-auth`: password hashing and JWT issue/verify/refresh.
  - `belhana-core`: planning scenarios, rules, costing and date/parse behaviour.
- `cargo check --workspace` → compiles cleanly with no warnings.
- `npm run build` in `apps/web` → SvelteKit static SPA builds to `apps/web/build` (non-blocking a11y
  label warnings only).
- `cargo check` in `apps/desktop/src-tauri` → Tauri 2 shell checks cleanly (icon.ico generated).

## Manual smoke scripts (`scripts/`)
Run the API against a throwaway SQLite file, then execute the PowerShell scripts. Arabic payloads are
kept in `.json` files because PowerShell can misread BOM-less UTF-8 scripts. Remember to assign the
token to a variable (`$tok = $reg.access_token`) — PowerShell does not expand object properties inside
a double-quoted string.

Exercised flows: register/login/refresh rotation; household/members/rules/budget status/what-if;
meal-plan generate/save/read; shopping generation/complete; inventory add-lot/valuation; price review
(`confirm_unit`); mark-prepared; occasion menus/approve/shopping/actual-vs-planned; recipe cost with
unknown prices; batch guides; merchants; import parse preview; AI manual mode.

## Browser E2E (2026-11-17)
With the seeded API and `vite dev` running, a headless browser exercised login → dashboard → bottom
navigation; refreshed the dashboard and confirmed the session persists; visited `/plan`, `/shopping`,
`/inventory`, `/recipes`, `/more` and all nine `/more` subpages; opened an import batch and reviewed its
original post and observations; and toggled Arabic/English (including document direction). No console
errors remained. This pass found and fixed the Svelte 5 `children` prop, auth-restore ordering and
language-reactivity issues described in `CHANGELOG.md`.

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
Android/iOS, OCR/CSV imports, load/performance and a formal security audit.
