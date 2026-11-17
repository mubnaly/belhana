-- 0003: Product catalog, units, conversions
-- NOTE: products table is referenced by 0002 (food_allergies) as well; SQLite
-- enforces FKs at runtime with PRAGMA foreign_keys=ON regardless of creation order.

CREATE TABLE IF NOT EXISTS product_categories (
    id TEXT PRIMARY KEY,
    slug TEXT NOT NULL UNIQUE,
    name_ar TEXT NOT NULL,
    name_en TEXT NOT NULL,
    parent_id TEXT REFERENCES product_categories(id) ON DELETE SET NULL,
    sort INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS measurement_units (
    code TEXT PRIMARY KEY,
    name_ar TEXT NOT NULL,
    name_en TEXT NOT NULL,
    kind TEXT NOT NULL,            -- mass | volume | count | container | other
    can_convert INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS products (
    id TEXT PRIMARY KEY,
    category_id TEXT REFERENCES product_categories(id) ON DELETE SET NULL,
    name_ar TEXT NOT NULL,
    name_en TEXT,
    default_unit TEXT,
    comparison_unit TEXT,
    package_size REAL,
    package_unit TEXT,
    brand TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS product_aliases (
    id TEXT PRIMARY KEY,
    product_id TEXT NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    alias_ar TEXT,
    alias_en TEXT,
    source TEXT,
    UNIQUE (product_id, alias_ar)
);

CREATE TABLE IF NOT EXISTS unit_conversions (
    id TEXT PRIMARY KEY,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    from_unit TEXT NOT NULL,
    to_unit TEXT NOT NULL,
    factor REAL NOT NULL,
    note TEXT
);

CREATE INDEX IF NOT EXISTS idx_products_category ON products(category_id);
CREATE INDEX IF NOT EXISTS idx_products_name_ar ON products(name_ar);
