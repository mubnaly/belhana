# 04 — Family, rules and budget

## Household (`/api/household`)
Fields: name, country, city, area, currency, household size, shopping frequency, daily/weekly/monthly/
occasion/snacks/contingency budgets (piasters), preferred merchants, kitchen equipment, notes,
`price_policy` (`freshest_verified` | `cheapest_verified`) and `consumption_workflow`
(e.g. `use_freezer_first`).

## Members (`/api/family/members`)
Each member has a display name, age band, and structured `likes`, `dislikes`, `exclusions` and
`allergies`. The planner reads exclusions/allergies as hard constraints and likes/dislikes as soft
preferences. CRUD is household-scoped (`GET/POST /api/family/members`, `PUT/DELETE /api/family/members/{id}`).

## Rules (`/api/rules`)
Supported rule types: `meal_repeat_limit`, `dish_frequency_limit`, `dish_consecutive_limit`,
`min_legume_meals`, `use_freezer_first`. Priority must be `hard` or `soft`; parameters are stored as
JSON (e.g. `maximum_occurrences`, `period_days`, `minimum_per_week`, `product_id`). Hard violations
become plan conflicts; soft rules influence scoring.

### Guidance & purchasing rules (seeded by default)
Every household receives six idempotently seeded rules (`notes = 'system-default'`), all enabled:

| Type | Params | Effect |
| --- | --- | --- |
| `ai_instruction` | `{"sentence": "..."}` | A free-text sentence injected into the AI **system prompt only** so the assistant "thinks for it" (e.g. use leftovers first, rice as the usual lunch, bread for breakfast/dinner, buy spices by a fixed amount). It does not grant the AI any write power. |
| `meal_staple` ×3 | `{"period": "breakfast"\|"lunch"\|"dinner", "product_id": "..."}` | Marks the main staple per period (bread for breakfast/dinner, rice for lunch). Influences planner scoring and is named in the explanation. |
| `fixed_spend` | `{"product_id": "...", "amount_minor": 2000}` | Buys an item by a fixed amount of money (e.g. 20 EGP of salt) instead of a weight; produces a `buy_by_amount_minor` shopping line. |
| `leftover_carryover` | `{"carry_over": true}` | Unused stock carries into the next week; new lots default to `carry_over = true` and are used first. |

These rules can be toggled with `POST /api/rules/{id}/enabled` and edited/added like other rules. The
instruction sentence is guidance for the assistant, not a hard plan constraint; mutating AI actions
still require proposal approval.

## Budget (`/api/budget`)
- `GET /api/budget/status?period=weekly|monthly` → budget, actual spending, planned cost, remaining,
  and per-category actual detail.
- `GET /api/budget/history?days=30` → daily actuals for sparkline rendering.
- `POST /api/budget/what-if` → deterministic `N`-day estimated cost and fits/exceeds verdict.

Budgets are stored in integer piasters; the UI collects EGP and multiplies by 100 before sending.
