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

## Budget (`/api/budget`)
- `GET /api/budget/status?period=weekly|monthly` → budget, actual spending, planned cost, remaining,
  and per-category actual detail.
- `GET /api/budget/history?days=30` → daily actuals for sparkline rendering.
- `POST /api/budget/what-if` → deterministic `N`-day estimated cost and fits/exceeds verdict.

Budgets are stored in integer piasters; the UI collects EGP and multiplies by 100 before sending.
