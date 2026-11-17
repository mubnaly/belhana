# 09 — Meal planning engine

`belhana_core::planning` is deterministic: the same inputs produce the same plan.

## Inputs
Household size and budget, member likes/dislikes/exclusions/allergies, hard/soft rules, known
inventory (including freezer-first workflow), verified prices under the household price policy, and
the recipe catalogue with meal-type tags.

## Output
7-day plan with breakfast/lunch/dinner slots, recipe choices, per-meal and total estimated cost
(integer piasters), an explanation per choice and a conflict list (e.g. hard-rule violation or
over-budget). Generating returns an in-memory plan; saving persists it, and reading a saved plan
restores slot ids so meals can be locked or marked prepared.

## Algorithm summary
1. Filter recipes by meal type and hard exclusions.
2. Score candidates by likes, dislikes, repetition limits, legume minimum, freezer-first availability
   and cost; hard rules are enforced, soft rules influence score.
3. Allocate slots greedily in deterministic order; attach cost from best verified price (or mark unknown).
4. Compare total against budget and emit explanations/conflicts.

## Editing
Locked meals stay fixed across regeneration. `mark_prepared` consumes stock and records the movement.
The UI (`/plan`) shows generate → review → save, conflicts, cost and the prepared state.
