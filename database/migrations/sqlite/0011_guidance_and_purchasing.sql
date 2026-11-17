-- 0011: Household guidance, leftovers, meal tips and fixed-spend purchasing

-- Unsigned stock remains available next week by default. carry_over=0 opts a
-- lot out (for example, an item reserved for a one-off occasion).
ALTER TABLE stock_lots ADD COLUMN carry_over INTEGER NOT NULL DEFAULT 1;

-- A shopping line can be "spend up to this amount" instead of "buy this qty"
-- (spices are typically bought by money, e.g. 20 EGP of salt).
ALTER TABLE shopping_list_items ADD COLUMN buy_by_amount_minor INTEGER;

CREATE TABLE IF NOT EXISTS meal_tips (
    id TEXT PRIMARY KEY,
    household_id TEXT REFERENCES households(id) ON DELETE CASCADE,
    title_ar TEXT NOT NULL,
    title_en TEXT NOT NULL,
    category TEXT NOT NULL,
    body_ar TEXT NOT NULL,
    body_en TEXT NOT NULL,
    sort INTEGER NOT NULL DEFAULT 0,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_meal_tips_household ON meal_tips(household_id);
CREATE INDEX IF NOT EXISTS idx_meal_tips_category ON meal_tips(category);
