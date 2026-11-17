# 05 — Products and prices

## Catalog (`/api/catalog`)
17 seeded categories and ~85 products (vegetables, fruits, poultry, meat, dairy, legumes, grains,
spices, bakery, frozen goods…). Products store Arabic/English names, default/comparison unit,
package size/unit, brand, aliases and active flag. Units/helpers in `belhana-common::units` handle
Arabic words (كيلو, جرام, لتر, قطعة…) and package conversions.

## Merchants (`/api/merchants`)
Name, slug, type, phone, Facebook/website URLs and notes. The Olad Zain merchant is seeded from the
user-supplied post; unspecified city/area and promotional claims are explicitly left unverified.

## Price observations (`/api/prices`)
A price is always: product (nullable until mapped), optional variant, merchant, location, integer
`amount_minor`, currency, unit + `unit_status`, ISO observation date, source type/URL, raw text,
confidence, review status and audit fields. Create/list/get/review endpoints exist; history is queried
by product, merchant, status or batch.

## Review actions
`verify`, `reject`, `confirm_unit {unit}`, `confirm_date {observation_date}`,
`map_product {product_id}`, `mark_stale`. Each action is audited. Confirming a unit/date/product resets
status to `unreviewed` so a human can still verify the corrected row — no automatic trust.

## Best-price selection
`get_best_price(pool, household_id, product_id, ...)`, used by costing/planning/shopping, selects only
verified observations according to the household `price_policy` and reports freshness. Unknown prices
return `null` (shown as unknown) instead of being estimated.

## Baselines
Seven user-supplied baseline prices are seeded into a separate table (`user_baselines`) as initial
planning references. They are never merged into merchant observation history.
