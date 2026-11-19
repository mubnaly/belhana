-- 0012: approximate product nutrition used for calorie estimates.
-- Values are editorial estimates per 100 g (or per 100 ml for liquids) and the
-- typical edible weight of one default unit (piece/package/bundle/carton...).
-- They are shown as estimates, never as verified lab results.

ALTER TABLE products ADD COLUMN kcal_per_100g REAL;
ALTER TABLE products ADD COLUMN grams_per_unit REAL;
