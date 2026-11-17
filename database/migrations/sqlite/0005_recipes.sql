-- 0005: Recipes, nutrition, feedback, batch-preparation guides

CREATE TABLE IF NOT EXISTS recipes (
    id TEXT PRIMARY KEY,
    household_id TEXT REFERENCES households(id) ON DELETE CASCADE,  -- NULL = system/global recipe
    name_ar TEXT NOT NULL,
    name_en TEXT,
    description TEXT,
    cuisine TEXT NOT NULL DEFAULT 'egyptian',
    meal_types TEXT NOT NULL DEFAULT '[]',   -- JSON array: breakfast|lunch|dinner|snack|dessert|juice
    difficulty TEXT NOT NULL DEFAULT 'easy', -- easy | medium | advanced
    prep_minutes INTEGER NOT NULL DEFAULT 10,
    cook_minutes INTEGER NOT NULL DEFAULT 20,
    default_servings INTEGER NOT NULL DEFAULT 4,
    cooking_method TEXT,                     -- fried | baked | boiled | grilled | stewed | raw | air_fried
    equipment TEXT,                          -- JSON array
    substitutions TEXT,                      -- JSON [{ingredient_product_id, substitute_product_id, note}]
    allergens TEXT,                          -- JSON array of allergen names
    storage_notes TEXT,
    source_type TEXT NOT NULL DEFAULT 'original',  -- original | user | external
    source_ref TEXT,
    license TEXT,
    child_portion_factor REAL NOT NULL DEFAULT 0.6, -- used in occasion scaling; transparent & editable
    is_active INTEGER NOT NULL DEFAULT 1,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS recipe_ingredients (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE SET NULL,
    raw_name_ar TEXT,
    raw_name_en TEXT,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    optional INTEGER NOT NULL DEFAULT 0,
    note TEXT,
    sort INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS recipe_steps (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    step_number INTEGER NOT NULL,
    text_ar TEXT NOT NULL,
    text_en TEXT,
    timer_minutes INTEGER
);

CREATE TABLE IF NOT EXISTS recipe_tags (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    tag TEXT NOT NULL,
    UNIQUE (recipe_id, tag)
);

CREATE TABLE IF NOT EXISTS recipe_nutrition_facts (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    calories_kcal REAL,
    protein_g REAL,
    carbs_g REAL,
    fat_g REAL,
    fiber_g REAL,
    sodium_mg REAL,
    per_serving INTEGER NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS recipe_nutrition_sources (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    source_name TEXT NOT NULL,
    source_url TEXT,
    retrieved_at TEXT
);

CREATE TABLE IF NOT EXISTS recipe_feedback (
    id TEXT PRIMARY KEY,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL,
    rating INTEGER,
    comment TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS batch_guides (
    id TEXT PRIMARY KEY,
    household_id TEXT REFERENCES households(id) ON DELETE CASCADE,
    title_ar TEXT NOT NULL,
    title_en TEXT,
    category TEXT,                    -- taameya | beans | sauce | vegetables | stock | freezer | leftovers | dessert
    batch_size_text TEXT,
    ingredients TEXT NOT NULL,        -- JSON [{product_id, name_ar, quantity, unit}]
    steps TEXT NOT NULL,              -- JSON [{text_ar}]
    portions INTEGER,
    storage TEXT,                     -- JSON {container, refrigerate, freeze, defrost, reheat}
    safety_source TEXT,
    safety_source_date TEXT,
    is_system INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_recipes_household ON recipes(household_id);
CREATE INDEX IF NOT EXISTS idx_recipe_ingredients_recipe ON recipe_ingredients(recipe_id);
