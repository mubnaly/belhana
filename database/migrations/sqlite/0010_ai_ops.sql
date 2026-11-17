-- 0010: AI layer, import jobs, audit logs, notifications

CREATE TABLE IF NOT EXISTS ai_providers (
    id TEXT PRIMARY KEY,
    provider TEXT NOT NULL,          -- openai_compatible | gemini | local
    model TEXT,
    base_url TEXT,
    api_key_env TEXT,
    is_active INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE IF NOT EXISTS ai_conversations (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_messages (
    id TEXT PRIMARY KEY,
    conversation_id TEXT NOT NULL REFERENCES ai_conversations(id) ON DELETE CASCADE,
    role TEXT NOT NULL,              -- user | assistant | tool | system
    content TEXT,
    tool_payload TEXT,               -- JSON for tool calls/results
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_usage_records (
    id TEXT PRIMARY KEY,
    household_id TEXT,
    user_id TEXT,
    provider TEXT NOT NULL,
    model TEXT,
    kind TEXT NOT NULL,
    prompt_tokens INTEGER,
    completion_tokens INTEGER,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS ai_action_proposals (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    user_id TEXT,
    action_type TEXT NOT NULL,       -- add_rule | replace_meal | set_budget | add_preference | ...
    payload TEXT NOT NULL,           -- JSON validated payload for review
    summary TEXT,
    status TEXT NOT NULL DEFAULT 'pending', -- pending | approved | rejected | applied
    result TEXT,
    created_at TEXT NOT NULL,
    resolved_at TEXT
);

CREATE TABLE IF NOT EXISTS import_jobs (
    id TEXT PRIMARY KEY,
    household_id TEXT,
    kind TEXT NOT NULL,              -- price_post_text | csv | screenshot | file
    status TEXT NOT NULL DEFAULT 'pending',  -- pending | running | done | failed
    payload TEXT,
    result TEXT,
    created_at TEXT NOT NULL,
    finished_at TEXT
);

CREATE TABLE IF NOT EXISTS audit_logs (
    id TEXT PRIMARY KEY,
    household_id TEXT,
    actor_user_id TEXT,
    action TEXT NOT NULL,
    entity_type TEXT NOT NULL,
    entity_id TEXT,
    before TEXT,
    after TEXT,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS notifications (
    id TEXT PRIMARY KEY,
    household_id TEXT NOT NULL REFERENCES households(id) ON DELETE CASCADE,
    kind TEXT NOT NULL,              -- expiry_reminder | price_update | plan_ready | system
    title TEXT NOT NULL,
    body TEXT,
    read INTEGER NOT NULL DEFAULT 0,
    created_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_proposals_household ON ai_action_proposals(household_id);
