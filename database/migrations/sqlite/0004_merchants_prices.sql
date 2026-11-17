-- 0004: Merchants, sources, price observations & review workflow

CREATE TABLE IF NOT EXISTS merchants (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    slug TEXT UNIQUE,
    type TEXT NOT NULL DEFAULT 'grocery',
    verification_status TEXT NOT NULL DEFAULT 'unverified',  -- unverified | verified
    phone TEXT,
    facebook_url TEXT,
    website_url TEXT,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS merchant_locations (
    id TEXT PRIMARY KEY,
    merchant_id TEXT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    city TEXT,
    area TEXT,
    address TEXT,
    is_confirmed INTEGER NOT NULL DEFAULT 0,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS merchant_sources (
    id TEXT PRIMARY KEY,
    merchant_id TEXT NOT NULL REFERENCES merchants(id) ON DELETE CASCADE,
    source_type TEXT NOT NULL,     -- facebook_page | facebook_group | website | manual | api
    url TEXT,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS price_source_posts (
    id TEXT PRIMARY KEY,
    merchant_id TEXT REFERENCES merchants(id) ON DELETE SET NULL,
    raw_text TEXT NOT NULL,
    source_type TEXT NOT NULL,     -- user_provided_facebook_post_text | manual | csv | screenshot_ocr | api
    source_url TEXT,
    original_date_text TEXT,
    normalized_date TEXT,
    date_status TEXT NOT NULL DEFAULT 'unreviewed', -- confirmed | needs_date_confirmation | rejected
    weekday_mismatch INTEGER NOT NULL DEFAULT 0,
    location_status TEXT NOT NULL DEFAULT 'unknown', -- unknown | confirmed | needs_confirmation
    captured_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS price_import_batches (
    id TEXT PRIMARY KEY,
    source_post_id TEXT REFERENCES price_source_posts(id) ON DELETE CASCADE,
    merchant_id TEXT REFERENCES merchants(id) ON DELETE SET NULL,
    batch_status TEXT NOT NULL DEFAULT 'imported_needs_review', -- imported_needs_review | in_review | approved | rejected
    created_by TEXT,
    created_at TEXT NOT NULL,
    reviewed_at TEXT,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS price_observations (
    id TEXT PRIMARY KEY,
    product_id TEXT REFERENCES products(id) ON DELETE SET NULL,
    product_variant TEXT,
    merchant_id TEXT REFERENCES merchants(id) ON DELETE SET NULL,
    location_id TEXT REFERENCES merchant_locations(id) ON DELETE SET NULL,
    amount_minor INTEGER,                  -- NULL when post mentions product without price
    currency TEXT NOT NULL DEFAULT 'EGP',
    unit TEXT,
    unit_status TEXT NOT NULL DEFAULT 'unresolved',   -- unresolved | confirmed | rejected
    package_size REAL,
    package_unit TEXT,
    observation_date TEXT,
    submitted_at TEXT NOT NULL,
    source_type TEXT NOT NULL,
    source_url TEXT,
    raw_text TEXT,
    extracted_text TEXT,
    raw_product_label TEXT,
    confidence REAL,
    review_status TEXT NOT NULL DEFAULT 'unreviewed',
    -- unreviewed | needs_unit_confirmation | needs_date_confirmation | needs_product_mapping | verified | rejected | stale | estimate
    previous_amount_minor INTEGER,
    promotion_note TEXT,
    reviewed_by TEXT,
    reviewed_at TEXT,
    batch_id TEXT REFERENCES price_import_batches(id) ON DELETE SET NULL,
    baseline_owner_user_id TEXT,           -- set when this is a user-supplied editable baseline estimate
    occurrence_index INTEGER NOT NULL DEFAULT 0,
    notes TEXT
);

CREATE TABLE IF NOT EXISTS price_review_events (
    id TEXT PRIMARY KEY,
    observation_id TEXT NOT NULL REFERENCES price_observations(id) ON DELETE CASCADE,
    action TEXT NOT NULL,          -- created | verified | rejected | unit_confirmed | date_confirmed | product_mapped | marked_stale | edited
    actor_user_id TEXT,
    from_status TEXT,
    to_status TEXT,
    note TEXT,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_prices_product ON price_observations(product_id);
CREATE INDEX IF NOT EXISTS idx_prices_merchant ON price_observations(merchant_id);
CREATE INDEX IF NOT EXISTS idx_prices_batch ON price_observations(batch_id);
