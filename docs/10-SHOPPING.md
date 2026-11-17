# 10 — Shopping

## Generation
`POST /api/shopping-lists` consolidates a saved plan (or explicit recipe selections) into one
list. Ingredient quantities are summed by product/comparison unit, converted with package sizes and
rounded up to purchasable packs. Each line carries estimated quantity, unit, unit price, estimated
total, price freshness and merchant when known. Unknown prices remain flagged, not guessed.

## Purchase flow
1. User reviews quantities and enters purchased quantity and actual price per line.
2. `POST /api/shopping/{id}/complete` records the actual spend, creates price observations from the
   actual prices (as fresh evidence, respecting the review workflow) and adds purchased quantities
   to inventory lots.
3. `GET /api/shopping/{id}/comparison` reports estimated vs actual cost.

## UI
`/shopping` shows the active list, editable purchased quantity/actual price fields, total estimate,
freshness warnings and the complete action. A completed list can be compared with the estimate.
