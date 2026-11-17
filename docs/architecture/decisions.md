# Architecture decisions (Phase 0)

## ADR-001 — SQLite first, Postgres-compatible schema
**Decision:** SQLx + SQLite for development; keep SQL portable (TEXT ids, JSON as TEXT, no SQLite-only
column types) and document the Postgres switch in `docs/18-DEPLOYMENT.md`.
**Why:** Docker/Postgres are unavailable in this environment; SQLite gives a fully runnable deliverable
with zero external services. **Consequence:** connection code and a few `COALESCE`/date helpers need a
verification pass before production Postgres use.

## ADR-002 — Crate consolidation
**Decision:** 19 crate sketch → 6 layers (`common`, `db`, `auth`, `core`, `ai`, `api`) + worker.
**Why:** feature-per-crate would fragment closely coupled domain logic and slow compilation without
improving boundaries. `core` keeps clearly separated modules and public functions per domain.

## ADR-003 — Integer piasters everywhere
**Decision:** `i64` minor units in DB, JSON, and domain functions; divide by 100 only for display.
**Why:** no floating-point money drift in budgets, plans or price comparisons.

## ADR-004 — No Tailwind
**Decision:** plain CSS with variables in `apps/web/src/app.css`.
**Why:** avoids an extra build dependency; the UI is small and RTL-first.

## ADR-005 — AI cannot mutate domain data
**Decision:** read-only tools execute; mutating tool calls insert `ai_action_proposals` rows for human approval.
**Why:** prices, plans and shopping state must remain deterministic and traceable.

## ADR-006 — Import ambiguity is preserved
**Decision:** pasted merchant posts are stored verbatim in `price_source_posts`; parsing produces
*proposed* dates/units with flags and never auto-verifies.
**Why:** user supplied post has missing units and a weekday/date inconsistency; silently fixing it
would corrupt price evidence.

## Unresolved requirements
- Postgres production deployment / backups.
- Live AI provider validation with a real API key.
- Official Meta/Facebook data access, OCR and CSV/Excel import.
- Android/iOS packaging and offline synchronization.
