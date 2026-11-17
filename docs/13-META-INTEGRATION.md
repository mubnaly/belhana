# 13 — Meta / merchant price integrations

## Implemented: paste-and-parse
The supported first integration is **manual paste of public post text** into `/more/imports`:
`POST /api/imports/parse` (dry run) and `POST /api/imports/batches` (persist unreviewed batch).
The original text, proposed date, weekday mismatch and per-line flags are retained. See `docs/06`.

## Not implemented (deliberate blockers)
- Official Facebook/Meta Graph API: requires app review, permission to read page posts and a stable
  token. No approved access exists in this environment, so no scraping or token-based workaround is used.
- Screenshot/OCR extraction: no OCR dependency integrated; OCR output would still require human review
  and would need its own ambiguity flags.
- CSV/Excel uploads and merchant-submitted feeds: not built; the paste format is the only ingestion path.

## Design for future connectors
Any future connector must: store the raw source verbatim, produce proposed (never final) units/dates,
attach source URL and captured-at, keep every row `unreviewed` until a human reviews it, and reuse the
existing `price_source_posts` / `price_import_batches` / `price_observations` review workflow.
Community posts are treated as claims, not verified facts.
