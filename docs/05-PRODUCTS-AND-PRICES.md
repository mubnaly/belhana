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

### Manual entry & packages
`POST /api/prices` also accepts a fully manual observation (`source_type: "manual"`, `unit_status:
"confirmed"`). Optional `package_size` and `package_unit` describe the pack, so eggs can be recorded as
30 pieces for 130 EGP (`amount_minor = 13000`, `unit = "piece"`, `package_size = 30`, `package_unit =
"piece"`) or 15 pieces for 65 EGP. A price with no unit still lands in `needs_unit_confirmation` and is
excluded from verified costing.

### Editing a confirmed price
`PUT /api/prices/{id}` updates the amount/unit/date/package of an existing observation. The prior value
is retained in `previous_amount_minor` and an audited `price_review_events` row with `action = 'edited'`
is written — the price history stays traceable rather than being silently overwritten.

## Review actions
`verify`, `reject`, `confirm_unit {unit}`, `confirm_date {observation_date}`,
`map_product {product_id}`, `mark_stale`, plus `edited` from the update endpoint. Each action is audited.
Confirming a unit/date/product resets status to `unreviewed` so a human can still verify the corrected
row — no automatic trust.

## Best-price selection
`get_best_price(pool, household_id, product_id, ...)`, used by costing/planning/shopping, selects only
verified observations according to the household `price_policy` and reports freshness. Under
`cheapest_verified`, all usable observations are compared in **one reference unit** — the product's
`package_unit` when a package size is set, otherwise its `default_unit` (else `piece`). Package totals
are divided by `package_size`, so 15 eggs for 65 EGP can correctly beat 30 for 130 EGP. Observations that
cannot be converted into the reference unit are skipped rather than guessed (`carton` has no universal
conversion to `piece` without package metadata). Unknown prices return `null` (shown as unknown) instead
of being estimated.

## Baselines
Seven user-supplied baseline prices are seeded into a separate table (`user_baselines`) as initial
planning references. They are never merged into merchant observation history.
