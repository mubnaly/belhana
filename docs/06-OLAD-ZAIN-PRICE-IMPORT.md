# 06 — Olad Zain price import

The user supplied the verbatim text of a Facebook price post from «محلات أولاد زين للخضروات…».
It is stored unchanged in `price_source_posts.raw_text`; parsing only proposes structure.

## What the importer does
- `POST /api/imports/parse` → dry-run `ParsedPost`: original/normalized text, `original_date_text`,
  `proposed_date`, `weekday_word`, `weekday_mismatch`, and parsed lines with prices, previous prices,
  unit words, proposed units/quantities and flags.
- `POST /api/imports/batches` → persists an `import_batch` plus one `price_observations` row per
  parsed price, all `unreviewed`, linked back to the source post and batch.
- `GET /api/imports/batches`, `/{id}`, `/{id}/observations`, `/{id}/source-post` → review UI data.
- `POST /api/imports/batches/{id}/confirm-date` → human confirms/rejects the proposed date.
- `POST /api/imports/batches/{id}/status` → `in_review` / `approved` / `rejected` / reopened.
  Approval closes the workflow; it **never** verifies ambiguous rows.

## Seeded result
- 89 source lines → 96 price observations.
- 67 require unit confirmation (`unit_not_stated`), 5 require product mapping.
- Weekday text vs proposed date mismatch is flagged for human confirmation.
- Explicit quantities are parsed but flagged `explicit_quantity_confirm`.

## Why not auto-correct
Missing units, a weekday/date mismatch and unverifiable promotional claims are genuine ambiguities in
the source. Guessing would turn a community post into false verified price evidence. Therefore the
review screen surfaces each flag and requires explicit human decisions; every decision is audited.

## Integration limits
No official Meta/Facebook API access, no screenshot OCR and no CSV/Excel ingestion in this delivery
(see `docs/13-META-INTEGRATION.md`).
