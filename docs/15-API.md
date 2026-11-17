# 15 — API reference (implemented surface)

Base: `http://127.0.0.1:8080`. Health check: `GET /healthz` → `{"app":"belhana-api","ok":true}`.
All other endpoints require `Authorization: Bearer <access_token>` and are scoped to the token's household.
Errors: JSON `{"error":{"message":"..."}}` with an appropriate HTTP status. Money fields end in `_minor`
and are integer piasters; dates are `YYYY-MM-DD`.

## Endpoint groups (~85 handlers over 17 modules)
- **Auth** `/api/auth/register|login|refresh|logout|me|switch-household`
- **Household** `GET/PUT /api/household`
- **Family** `/api/family/members[/{id}]`, `/preferences`, `/allergies`, `/exclusions`
- **Rules** `/api/rules[/{id}]`, `/api/rules/{id}/enabled`
- **Budget** `/api/budget/status`, `/history`, `/what-if`
- **Catalog** `/api/catalog/categories[/{id}]`, `/api/catalog/products[/{id}]`
- **Prices** `GET/POST /api/prices`, `GET/PUT /api/prices/{id}`, `/{id}/review`, `/baseline`, `/best/{product_id}`.
  `POST` accepts `package_size`/`package_unit` (e.g. 30, `piece`); `PUT` edits an observation, preserving
  `previous_amount_minor` and auditing `action='edited'`. `/best/{product_id}` compares verified prices
  in the product's reference unit (pack unit when a package size is set), never mixing units.
- **Merchants** `/api/merchants[/{id}]`, `/api/merchants/{id}/locations`
- **Imports** `/api/imports/parse`, `/api/imports/batches[/{id}]`, `/{id}/observations`,
  `/{id}/source-post`, `/{id}/confirm-date`, `/{id}/status`
- **Recipes** `/api/recipes[/{id}]`, `/api/recipes/{id}/cost`, `/api/batch-guides[/{id}/convert]`
- **Inventory** `/api/inventory/locations`, `lots[/{id}]`, `/lots/{id}/adjust|move|carry-over`, `/consume`,
  `/waste`, `/stock/{product_id}`, `/valuation`, `/expiring`, `/leftovers`. Lots default to
  `carry_over = 1`; `POST /api/inventory/lots/{id}/carry-over {carry_over: bool}` toggles whether unused
  stock rolls into the next week (and is consumed first by the planner).
- **Planning** `GET/POST /api/meal-plans`, `/api/meal-plans/{id}`, `/api/planned-meals/{id}/prepared`.
  Explanations include staple fit and leftover reuse from the household rules.
- **Shopping** `/api/shopping-lists`, `/active`, `/{id}`, `/{id}/items/{item_id}`, `/{id}/complete`.
  `fixed_spend` rules add lines with `unit = "spend"`, `buy_by_amount_minor` (e.g. 2000 for 20 EGP of
  salt); completion records the actual expense but creates no fake `spend` stock lot.
- **Meal tips** `GET /api/meal-tips[?category=...]`, `GET /api/meal-tips/categories` — 25 editorial tips
  across 9 categories (`batch`, `bread`, `budget`, `leftovers`, `legumes`, `planning`, `prep`, `rice`, `spices`).
- **Occasions** `/api/events[/{id}]`, `/{id}/menus`, `/menus/approve/{menu_id}`, `/{id}/shopping`,
  `/{id}/actual-costs`, `/{id}/planned-vs-actual`
- **AI** `/api/ai/chat`, `/api/ai/status`, `/api/ai/proposals`, `/api/ai/proposals/{id}/resolve`
- **Ops** `/api/dashboard`, `/api/audit`, `/api/notifications`, `/healthz`

## Auth example
```powershell
$body = @{ email = 'test@belhana.app'; password = 'secret123' } | ConvertTo-Json
$r = Invoke-RestMethod http://127.0.0.1:8080/api/auth/login -Method Post -Body $body -ContentType 'application/json'
$tok = $r.access_token    # assign to its own variable: PowerShell expands "$r.access_token" incorrectly
$headers = @{ Authorization = "Bearer $tok" }
Invoke-RestMethod http://127.0.0.1:8080/api/household -Headers $headers
```
Refresh tokens rotate: using an old refresh token is rejected. On 401 the SPA refreshes once, then
logs out.

## Package price & edit example
```powershell
# 30 eggs for 130 EGP, comparable per-piece with a 15-egg pack
$price = @{ product_id=$eggsId; amount_minor=13000; unit='piece'; package_size=30;
            package_unit='piece'; observation_date='2026-11-17'; unit_status='confirmed' } | ConvertTo-Json
$o = Invoke-RestMethod "$base/api/prices" -Method Post -Headers $headers -ContentType 'application/json' -Body $price
Invoke-RestMethod "$base/api/prices/$($o.id)/review" -Method Post -Headers $headers -Body '{"action":"verify"}' -ContentType 'application/json'
Invoke-RestMethod "$base/api/prices/best/$eggsId" -Headers $headers
# Correct the price: prior value kept in previous_amount_minor, edit audited
Invoke-RestMethod "$base/api/prices/$($o.id)" -Method Put -Headers $headers -ContentType 'application/json' `
  -Body '{"amount_minor":15000,"unit":"piece","package_size":30,"package_unit":"piece"}'
```

## Client base URL
`apps/web/src/lib/api.js` uses the Vite proxy during development, `BELHANA_API_BASE` (or
`window.__BELHANA_API_BASE__`) when provided, and `http://127.0.0.1:8080` inside the Tauri WebView.
