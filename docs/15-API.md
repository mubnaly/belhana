# 15 — API reference (implemented surface)

Base: `http://127.0.0.1:8080`. Health check: `GET /healthz` → `{"app":"belhana-api","ok":true}`.
All other endpoints require `Authorization: Bearer <access_token>` and are scoped to the token's household.
Errors: JSON `{"error":{"message":"..."}}` with an appropriate HTTP status. Money fields end in `_minor`
and are integer piasters; dates are `YYYY-MM-DD`.

## Endpoint groups (~80 handlers over 16 modules)
- **Auth** `/api/auth/register|login|refresh|logout|me|switch-household`
- **Household** `GET/PUT /api/household`
- **Family** `/api/family/members[/{id}]`, `/preferences`, `/allergies`, `/exclusions`
- **Rules** `/api/rules[/{id}]`, `/api/rules/{id}/enabled`
- **Budget** `/api/budget/status`, `/history`, `/what-if`
- **Catalog** `/api/catalog/categories[/{id}]`, `/api/catalog/products[/{id}]`
- **Prices** `GET/POST /api/prices`, `/api/prices/{id}`, `/{id}/review`, `/baseline`, `/best/{product_id}`
- **Merchants** `/api/merchants[/{id}]`, `/api/merchants/{id}/locations`
- **Imports** `/api/imports/parse`, `/api/imports/batches[/{id}]`, `/{id}/observations`,
  `/{id}/source-post`, `/{id}/confirm-date`, `/{id}/status`
- **Recipes** `/api/recipes[/{id}]`, `/api/recipes/{id}/cost`, `/api/batch-guides[/{id}/convert]`
- **Inventory** `/api/inventory/locations`, `lots[/{id}]`, `/lots/{id}/adjust|move`, `/consume`,
  `/waste`, `/stock/{product_id}`, `/valuation`, `/expiring`
- **Planning** `GET/POST /api/meal-plans`, `/api/meal-plans/{id}`, `/api/planned-meals/{id}/prepared`
- **Shopping** `/api/shopping-lists`, `/active`, `/{id}`, `/{id}/items/{item_id}`, `/{id}/complete`
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

## Client base URL
`apps/web/src/lib/api.js` uses the Vite proxy during development, `BELHANA_API_BASE` (or
`window.__BELHANA_API_BASE__`) when provided, and `http://127.0.0.1:8080` inside the Tauri WebView.
