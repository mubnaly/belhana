-- 0013: approximate macronutrients for the product catalog.
-- Grams per 100 g (or 100 ml for liquids), paired with the existing calorie
-- estimate. Editorial household estimates only; the UI labels them as such.

ALTER TABLE products ADD COLUMN protein_per_100g REAL;
ALTER TABLE products ADD COLUMN carbs_per_100g REAL;
ALTER TABLE products ADD COLUMN fat_per_100g REAL;
ALTER TABLE products ADD COLUMN fiber_per_100g REAL;
