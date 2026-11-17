-- 0007: Meal planning

CREATE TABLE IF NOT EXISTS meal_plans (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    start_date TEXT NOT NULL,
    end_date TEXT NOT NULL,
    days INTEGER NOT NULL,
    status TEXT NOT NULL DEFAULT 'draft',   -- draft | accepted | archived | template
    budget_amount_minor INTEGER,
    servings_per_meal INTEGER NOT NULL DEFAULT 4,
    periods_enabled TEXT NOT NULL,          -- JSON array of enabled periods
    shopping_pattern TEXT NOT NULL DEFAULT 'weekly',
    estimated_total_cost_minor INTEGER,
    created_by TEXT,
    created_at TEXT NOT NULL,
    accepted_at TEXT
);

CREATE TABLE IF NOT EXISTS meal_plan_days (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES meal_plans(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    day_index INTEGER NOT NULL,
    estimated_cost_minor INTEGER
);

CREATE TABLE IF NOT EXISTS planned_meals (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES meal_plans(id) ON DELETE CASCADE,
    day_id TEXT NOT NULL REFERENCES meal_plan_days(id) ON DELETE CASCADE,
    date TEXT NOT NULL,
    period TEXT NOT NULL,           -- breakfast | lunch | dinner | snack | dessert | juice
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    servings INTEGER NOT NULL,
    locked INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'proposed', -- proposed | replaced | prepared | skipped
    cost_estimate_minor INTEGER,
    inventory_used TEXT,            -- JSON [{product_id, quantity, unit, lot_id}]
    missing_ingredients TEXT,       -- JSON [{product_id, name_ar, quantity, unit}]
    explanation TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS planning_constraints (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES meal_plans(id) ON DELETE CASCADE,
    snapshot TEXT NOT NULL          -- JSON snapshot of engine inputs
);

CREATE TABLE IF NOT EXISTS planning_run_results (
    id TEXT PRIMARY KEY,
    plan_id TEXT NOT NULL REFERENCES meal_plans(id) ON DELETE CASCADE,
    run_at TEXT NOT NULL,
    success INTEGER NOT NULL,
    message TEXT,
    conflicts TEXT,                 -- JSON array of conflict explanations
    stats TEXT                      -- JSON summary stats
);

CREATE INDEX IF NOT EXISTS idx_plans_household ON meal_plans(household_id);
CREATE INDEX IF NOT EXISTS idx_planned_meals_plan ON planned_meals(plan_id);
