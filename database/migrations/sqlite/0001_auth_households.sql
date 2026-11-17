-- 0001: Identity & tenancy (users, households, memberships, devices, sessions)
-- IDs are TEXT UUIDs for Postgres compatibility.

CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    email TEXT NOT NULL UNIQUE COLLATE NOCASE,
    password_hash TEXT NOT NULL,
    display_name TEXT NOT NULL,
    locale TEXT NOT NULL DEFAULT 'ar',
    role TEXT NOT NULL DEFAULT 'user',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS households (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    country TEXT NOT NULL DEFAULT 'EG',
    city TEXT,
    area TEXT,
    currency TEXT NOT NULL DEFAULT 'EGP',
    household_size INTEGER NOT NULL DEFAULT 1,
    shopping_frequency TEXT NOT NULL DEFAULT 'weekly',
    daily_budget_minor INTEGER,
    weekly_budget_minor INTEGER,
    monthly_budget_minor INTEGER,
    occasion_budget_minor INTEGER,
    snacks_budget_minor INTEGER,
    contingency_minor INTEGER,
    preferred_merchants TEXT,           -- JSON array of merchant ids
    kitchen_equipment TEXT,             -- JSON array of strings
    notes TEXT,
    price_policy TEXT NOT NULL DEFAULT 'freshest_verified',
    consumption_workflow TEXT NOT NULL DEFAULT 'confirm_on_prepare',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS household_memberships (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    role TEXT NOT NULL DEFAULT 'member',
    created_at TEXT NOT NULL,
    UNIQUE (household_id, user_id)
);

CREATE TABLE IF NOT EXISTS devices (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    device_name TEXT,
    platform TEXT,
    last_seen_at TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL UNIQUE,
    device_id TEXT REFERENCES devices(id) ON DELETE SET NULL,
    created_at TEXT NOT NULL,
    expires_at TEXT NOT NULL,
    revoked_at TEXT
);

CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id);
CREATE INDEX IF NOT EXISTS idx_membership_user ON household_memberships(user_id);
