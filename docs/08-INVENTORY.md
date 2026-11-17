# 08 — Inventory

## Model
- Locations: pantry, fridge, freezer (seeded per household).
- Lots: product, quantity, unit, location, expiry date, estimated cost (piasters), notes, source
  (purchase/batch-prep), created time.
- Movements: add, consume, discard, adjust — every stock change is recorded, never a blind overwrite.

## API / UI
`/api/inventory/locations`, `/api/inventory/lots`, `/api/inventory/movements`,
`/api/inventory/valuation`, `/api/inventory/expiring`. The inventory screen lists lots with expiry
chips, allows adding a lot with product/location/quantity/expiry/cost, and shows total value.
Valuation uses `CAST(... AS INTEGER)` when reading SQLite REAL columns to keep integer piasters.

## Expiry reminders
`maintenance_worker --once` (or scheduled) creates a deduplicated `expiry_reminder` notification per
expiring lot, uses `BELHANA_STALE_DAYS` for the price sweep and prunes notifications older than 30 days.

## Interaction with other domains
- Planner prefers known stock and (with `use_freezer_first`) frozen lots before shopping.
- Shopping completion adds purchased quantities as new lots.
- Marking a planned meal prepared consumes lots through the movement log.
