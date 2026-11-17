-- 0009: Special occasions & family gatherings (العزومات والمناسبات)

CREATE TABLE IF NOT EXISTS events (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    name TEXT NOT NULL,
    occasion_type TEXT,
    date TEXT,
    time TEXT,
    adults INTEGER NOT NULL DEFAULT 0,
    children INTEGER NOT NULL DEFAULT 0,
    child_ages TEXT,                 -- JSON array of age bands
    budget_total_minor INTEGER,
    budget_per_guest_minor INTEGER,
    meal_type TEXT,                  -- lunch | dinner | breakfast | buffet | dessert_tea
    style TEXT,                      -- egyptian_traditional | economical | festive | syrian | mixed | custom
    restrictions TEXT,               -- JSON
    allergies TEXT,                  -- JSON
    equipment TEXT,                  -- JSON
    include_drinks INTEGER NOT NULL DEFAULT 1,
    include_sweets INTEGER NOT NULL DEFAULT 1,
    include_fruit INTEGER NOT NULL DEFAULT 0,
    include_appetizers INTEGER NOT NULL DEFAULT 1,
    use_inventory INTEGER NOT NULL DEFAULT 1,
    status TEXT NOT NULL DEFAULT 'draft',
    -- draft | menu_proposed | menu_approved | shopping | shopping_done | preparing | completed
    created_by TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS event_guests (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    name TEXT,
    kind TEXT NOT NULL DEFAULT 'adult',   -- adult | child
    age_band TEXT
);

CREATE TABLE IF NOT EXISTS event_preferences (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,              -- include | exclude
    recipe_id TEXT REFERENCES recipes(id) ON DELETE SET NULL,
    note TEXT
);

CREATE TABLE IF NOT EXISTS event_menus (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    option_index INTEGER NOT NULL,
    tier TEXT NOT NULL,              -- economical | balanced | elaborate
    title TEXT,
    total_cost_minor INTEGER,
    cost_per_guest_minor INTEGER,
    selected INTEGER NOT NULL DEFAULT 0,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS event_menu_dishes (
    id TEXT PRIMARY KEY,
    menu_id TEXT NOT NULL REFERENCES event_menus(id) ON DELETE CASCADE,
    recipe_id TEXT NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    course TEXT NOT NULL,            -- main | side | salad | appetizer | soup | dessert | drink
    servings INTEGER NOT NULL,
    scale REAL NOT NULL DEFAULT 1.0,
    cost_minor INTEGER,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS event_shopping_items (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    est_price_minor INTEGER,
    est_total_minor INTEGER,
    price_observation_id TEXT REFERENCES price_observations(id) ON DELETE SET NULL,
    purchased INTEGER NOT NULL DEFAULT 0,
    actual_price_minor INTEGER
);

CREATE TABLE IF NOT EXISTS event_actual_costs (
    id TEXT PRIMARY KEY,
    event_id TEXT NOT NULL REFERENCES events(id) ON DELETE CASCADE,
    category TEXT NOT NULL DEFAULT 'groceries',
    amount_minor INTEGER NOT NULL,
    note TEXT,
    date TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_events_household ON events(household_id);
