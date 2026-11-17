# 10 — Shopping

## Generation
`POST /api/shopping-lists` consolidates a saved plan (or explicit recipe selections) into one
list. Ingredient quantities are summed by product/comparison unit, converted with package sizes and
rounded up to purchasable packs. Each line carries estimated quantity, unit, unit price, estimated
total, price freshness and merchant when known. Unknown prices remain flagged, not guessed.

### Fixed-spend lines
A `fixed_spend` household rule (e.g. 2000 piasters of salt) adds a line with `unit = "spend"` and
`buy_by_amount_minor = 2000` — "buy this amount of money's worth" rather than a weight. The UI shows a
"اشترِ بمبلغ ~٢٠ ج.م" chip. This is how small spices are purchased without pretending a kilo is needed.

## Purchase flow
1. User reviews quantities and enters purchased quantity and actual price per line.
2. `POST /api/shopping-lists/{id}/complete` records the actual spend, creates price observations from
   actual prices (as fresh evidence, respecting the review workflow) and adds purchased quantities to
   inventory lots. Fixed-spend lines record their actual expense but create **no** fake `spend` stock
   lot — you bought 20 EGP of salt, not "1 spend" of inventory.
3. `GET /api/shopping/{id}/comparison` reports estimated vs actual cost.

## UI
`/shopping` shows the active list, editable purchased quantity/actual price fields, total estimate,
freshness warnings, fixed-spend chips and the complete action. A completed list can be compared with
the estimate.
