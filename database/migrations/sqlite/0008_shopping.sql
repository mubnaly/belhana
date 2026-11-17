-- 0008: Shopping lists, trips, purchases, actual expenses

CREATE TABLE IF NOT EXISTS shopping_lists (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    plan_id TEXT REFERENCES meal_plans(id) ON DELETE SET NULL,
    event_id TEXT,
    status TEXT NOT NULL DEFAULT 'draft',   -- draft | active | completed
    trip_date TEXT,
    policy TEXT NOT NULL DEFAULT 'freshest_verified',
    estimated_total_minor INTEGER,
    created_at TEXT NOT NULL,
    completed_at TEXT
);

CREATE TABLE IF NOT EXISTS shopping_list_items (
    id TEXT PRIMARY KEY,
    list_id TEXT NOT NULL REFERENCES shopping_lists(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    required_qty REAL NOT NULL,
    required_unit TEXT NOT NULL,
    stock_qty REAL,
    stock_unit TEXT,
    shopping_qty REAL,
    shopping_unit TEXT,
    package_size REAL,
    package_unit TEXT,
    estimated_unit_price_minor INTEGER,
    estimated_total_minor INTEGER,
    price_observation_id TEXT REFERENCES price_observations(id) ON DELETE SET NULL,
    merchant_id TEXT REFERENCES merchants(id) ON DELETE SET NULL,
    price_freshness TEXT,           -- fresh | stale | none
    purchased_qty REAL,
    purchased_unit TEXT,
    actual_price_minor INTEGER,
    purchased INTEGER NOT NULL DEFAULT 0,
    status TEXT NOT NULL DEFAULT 'pending',  -- pending | purchased | substituted | removed
    substitute_product_id TEXT REFERENCES products(id) ON DELETE SET NULL,
    notes TEXT,
    sort INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS shopping_trips (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    list_id TEXT REFERENCES shopping_lists(id) ON DELETE SET NULL,
    event_id TEXT,
    date TEXT NOT NULL,
    merchant_id TEXT REFERENCES merchants(id) ON DELETE SET NULL,
    total_planned_minor INTEGER,
    total_actual_minor INTEGER,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS purchase_items (
    id TEXT PRIMARY KEY,
    trip_id TEXT NOT NULL REFERENCES shopping_trips(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    actual_price_minor INTEGER,
    price_observation_id TEXT REFERENCES price_observations(id) ON DELETE SET NULL,
    lot_id TEXT REFERENCES stock_lots(id) ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS actual_expenses (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    category TEXT NOT NULL DEFAULT 'groceries',  -- groceries | occasion | dining | other
    amount_minor INTEGER NOT NULL,
    date TEXT NOT NULL,
    trip_id TEXT REFERENCES shopping_trips(id) ON DELETE SET NULL,
    event_id TEXT,
    note TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lists_household ON shopping_lists(household_id);
CREATE INDEX IF NOT EXISTS idx_list_items_list ON shopping_list_items(list_id);
