-- 0002: Family members, preferences, allergies/exclusions, household rules

CREATE TABLE IF NOT EXISTS family_members (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    display_name TEXT NOT NULL,
    age_band TEXT,
    is_active INTEGER NOT NULL DEFAULT 1,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS member_preferences (
    id TEXT PRIMARY KEY,
    member_id TEXT NOT NULL REFERENCES family_members(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,            -- like | dislike | preferred_ingredient | preferred_dish | preferred_method | dietary
    ref_type TEXT NOT NULL,        -- ingredient | dish | method | text
    ref_id TEXT,                   -- product or recipe id when ref_type is not text
    text_value TEXT,
    strength TEXT NOT NULL DEFAULT 'mild',  -- strong | mild
    raw_text TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS food_allergies (
    id TEXT PRIMARY KEY,
    member_id TEXT NOT NULL REFERENCES family_members(id) ON DELETE CASCADE,
    product_id TEXT REFERENCES products(id) ON DELETE SET NULL,
    allergen TEXT,
    severity TEXT NOT NULL DEFAULT 'severe',   -- severe | moderate | unknown
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS food_exclusions (
    id TEXT PRIMARY KEY,
    household_id TEXT REFERENCES households(id) ON DELETE CASCADE,
    member_id TEXT REFERENCES family_members(id) ON DELETE SET NULL,
    product_id TEXT REFERENCES products(id) ON DELETE CASCADE,
    reason TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS household_rules (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    rule_type TEXT NOT NULL,       -- meal_repeat_limit | dish_consecutive_limit | min_legume_meals | use_freezer_first | member_dislike_avoid | allergen_ban | dish_frequency_limit
    priority TEXT NOT NULL DEFAULT 'soft',  -- hard | soft
    params TEXT NOT NULL,          -- JSON params per rule type
    enabled INTEGER NOT NULL DEFAULT 1,
    notes TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_history (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    period TEXT NOT NULL,          -- daily | weekly | monthly | trip | occasion | snacks
    amount_minor INTEGER NOT NULL,
    period_start TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS budget_allocations (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    category TEXT NOT NULL,
    amount_minor INTEGER NOT NULL,
    period TEXT NOT NULL DEFAULT 'weekly',
    UNIQUE (household_id, category, period)
);

CREATE INDEX IF NOT EXISTS idx_members_household ON family_members(household_id);
CREATE INDEX IF NOT EXISTS idx_rules_household ON household_rules(household_id);
