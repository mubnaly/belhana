-- 0015: user-entered prices are confirmed (and still editable) by default.
-- `confirmed` is deliberately distinct from `verified`: it records that the
-- user accepted the value in the app, not a laboratory or clinical review.
-- Imported/ambiguous prices keep their original status.
INSERT INTO price_review_events (id, observation_id, action, from_status, to_status, note, created_at)
SELECT lower(hex(randomblob(16))), id, 'confirmed', review_status, 'confirmed',
       'auto-confirmed on upgrade: user-entered price, still editable', datetime('now')
  FROM price_observations
 WHERE source_type = 'manual'
   AND unit_status = 'confirmed'
   AND review_status IN ('unreviewed', 'estimate');

UPDATE price_observations
   SET review_status = 'confirmed', reviewed_at = datetime('now')
 WHERE source_type = 'manual'
   AND unit_status = 'confirmed'
   AND review_status IN ('unreviewed', 'estimate');
