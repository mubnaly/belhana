# 01 — Product specification

## Mission
بالهنا (Belhana) helps Egyptian households decide **what to cook, what to buy and how much to spend**,
using traceable local price evidence and their own preferences. The product is Arabic-first, works on
web and desktop, and never invents prices or food facts.

## Users
- Budget-conscious parents planning a week of meals.
- Household cooks who need recipes, batch-cooking and storage guidance.
- Anyone comparing merchant price posts with uncertain units and dates.

## Core principles
1. Money is integer minor units (piasters); no floats.
2. A price is a dated observation with a source and a review status, never a bare number.
3. Ambiguous data (unit/date/product) is preserved and flagged, never silently corrected.
4. User baselines are separate from merchant quotes.
5. Planners/cost/shopping calculations are deterministic; AI proposes, humans approve.
6. Everything is household-scoped.

## Delivered scope (MVP)
Accounts + household, members/rules/budgets, catalog + merchants + price history, paste-and-review
price posts (Olad Zain seed), recipes + costing + batch guides, inventory, 7-day meal plan,
shopping + purchase workflow, occasions, AI assistant (tool loop with proposals), Arabic/English UI,
web + Tauri clients, maintenance worker.

## Explicitly out of scope for this delivery
Official Facebook/Meta API access, OCR/screenshot extraction, CSV/Excel import, Android/iOS packages,
offline sync/change queue, live AI provider validation, production Postgres deployment.

## MVP acceptance scenarios
See `docs/17-TESTING.md` for how scenarios A–H from the master prompt map to the implemented flows.
