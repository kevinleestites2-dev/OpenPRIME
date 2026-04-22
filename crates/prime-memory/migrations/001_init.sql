CREATE TABLE IF NOT EXISTS memories (
    id          TEXT PRIMARY KEY,
    kind        TEXT NOT NULL,
    content     TEXT NOT NULL,
    session_id  TEXT NOT NULL,
    agent_id    TEXT,
    tags        TEXT DEFAULT '[]',
    created_at  TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE VIRTUAL TABLE IF NOT EXISTS memories_fts USING fts5(
    id UNINDEXED,
    content,
    content='memories',
    content_rowid='rowid'
);

CREATE TABLE IF NOT EXISTS sessions (
    id          TEXT PRIMARY KEY,
    agent_id    TEXT NOT NULL,
    summary     TEXT,
    started_at  TEXT NOT NULL,
    ended_at    TEXT,
    message_count INTEGER DEFAULT 0,
    token_count   INTEGER DEFAULT 0
);

CREATE TABLE IF NOT EXISTS user_profile (
    key         TEXT PRIMARY KEY,
    value       TEXT NOT NULL,
    updated_at  TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS skill_refs (
    id          TEXT PRIMARY KEY,
    skill_name  TEXT NOT NULL,
    session_id  TEXT NOT NULL,
    helped      INTEGER DEFAULT 1,
    created_at  TEXT NOT NULL
);
