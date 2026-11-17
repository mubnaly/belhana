-- 0006: Inventory (storage locations, stock lots, movements, waste)

CREATE TABLE IF NOT EXISTS storage_locations (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    name_ar TEXT NOT NULL,
    name_en TEXT,
    kind TEXT NOT NULL DEFAULT 'pantry',   -- pantry | fridge | freezer | other
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS stock_lots (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    location_id TEXT REFERENCES storage_locations(id) ON DELETE SET NULL,
    purchase_date TEXT,
    opened_date TEXT,
    expiry_date TEXT,
    source TEXT NOT NULL DEFAULT 'manual',  -- manual | shopping | batch | adjustment
    estimated_cost_minor INTEGER,
    notes TEXT,
    status TEXT NOT NULL DEFAULT 'active',  -- active | consumed | discarded
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS inventory_movements (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    lot_id TEXT NOT NULL REFERENCES stock_lots(id) ON DELETE CASCADE,
    movement_type TEXT NOT NULL,    -- add | consume | move | adjust | discard | waste
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    from_location_id TEXT,
    to_location_id TEXT,
    reason TEXT,
    related_shopping_item_id TEXT,
    related_planned_meal_id TEXT,
    created_by TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS food_waste_records (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE SET NULL,
    quantity REAL NOT NULL,
    unit TEXT NOT NULL,
    date TEXT NOT NULL,
    reason TEXT,
    estimated_cost_minor INTEGER,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_lots_household ON stock_lots(household_id);
CREATE INDEX IF NOT EXISTS idx_lots_product ON stock_lots(product_id);
CREATE INDEX IF NOT EXISTS idx_lots_expiry ON stock_lots(expiry_date);
