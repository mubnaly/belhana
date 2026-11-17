# 07 — Recipes and economical cooking

## Catalog
18 seeded Egyptian/Arab recipes (breakfasts, legume dishes, rice dishes, poultry, fish, soups,
salads, desserts) with ingredients (product link or free-text name, quantity, unit), steps, prep/cook
times, default servings, meal types and tags. The UI supports adding recipes and browsing details.

## Deterministic costing
`GET /api/recipes/{id}/cost?servings=N` (and the cost button in the UI) computes ingredient cost from
`get_best_price` under the household `price_policy`. If a product has no verified observation, the
line stays `null`; the response lists unknown ingredients. No average, guess or AI estimate is used.

## Batch preparation and storage
Nine batch-prep guides (`/api/cooking/guides`) cover economical cooking, legume soaking, rice/pasta
batches, poultry portioning, freezer rules, labelling and reheating. Guides are editorial content,
not generated from user data.

## Cooking workflow
Planned meals can be marked prepared (`POST /api/plan/meals/{id}/prepared`), which consumes inventory
lots using the household consumption workflow (freezer-first) and logs the movement.
