# 11 — Special occasions (العزومات والمناسبات)

## Setup
`POST /api/events` creates an event with name, date, type, adults, children and budget (piasters).
Additional guest types can be modelled in the payload. Events are household-scoped.

## Menu generation
`POST /api/events/{id}/menus` produces three tiers — `economical`, `balanced`, `elaborate` — with
recipe scaling from guest counts, per-person and total costs, and unknowns surfaced. The user selects
a tier; `POST /api/events/{id}/approve` records the decision. `GET /api/events/{id}/shopping` builds
the event shopping list from the approved menu.

## Execution
Preparation checklist steps come from recipe steps and batch guides. `POST /api/events/{id}/actual-cost`
records spend; the event report compares planned vs actual totals.

## UI
`/more/occasions` supports create event → generate menus → select tier → approve → view shopping list →
record actual cost and see the planned-vs-actual comparison.

## Scope note
There is no separate local-culture suggestion engine beyond the seeded menu catalogue; local occasion
ideas are represented as seeded recipes and editorial guidance.
