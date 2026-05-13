-- ============================================================
-- Migration 0002: Normalized schema
-- ============================================================
-- Keeps app_sessions / app_rules from 0001 (legacy, read-only).
-- All new writes go to the tables below.
-- ============================================================

-- ----------------------------------------------------------
-- categories: productivity classification labels
-- ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS categories (
    id           INTEGER PRIMARY KEY,
    name         TEXT    NOT NULL UNIQUE,
    color        TEXT    NOT NULL DEFAULT '#94a3b8',
    is_productive INTEGER NOT NULL DEFAULT 0
                          CHECK (is_productive IN (0, 1))
);

INSERT OR IGNORE INTO categories (id, name, color, is_productive) VALUES
    (1, 'Productive',  '#22c55e', 1),
    (2, 'Neutral',     '#94a3b8', 0),
    (3, 'Distraction', '#ef4444', 0);

-- ----------------------------------------------------------
-- apps: registry of seen executables with running totals
-- ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS apps (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    exe          TEXT    NOT NULL UNIQUE,
    display_name TEXT    NOT NULL,
    exe_path     TEXT    NOT NULL DEFAULT '',
    category_id  INTEGER NOT NULL DEFAULT 2
                         REFERENCES categories(id) ON UPDATE CASCADE,
    first_seen   TEXT    NOT NULL,
    last_seen    TEXT    NOT NULL,
    total_ms     INTEGER NOT NULL DEFAULT 0
                         CHECK (total_ms >= 0)
);

-- ----------------------------------------------------------
-- activity_sessions: one row per focus session
-- ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS activity_sessions (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    app_id      INTEGER NOT NULL REFERENCES apps(id) ON DELETE CASCADE,
    title       TEXT    NOT NULL,
    started_at  TEXT    NOT NULL,   -- RFC3339 UTC
    ended_at    TEXT    NOT NULL,   -- RFC3339 UTC
    duration_ms INTEGER NOT NULL CHECK (duration_ms > 0)
);

CREATE INDEX IF NOT EXISTS idx_act_started_at
    ON activity_sessions(started_at);

CREATE INDEX IF NOT EXISTS idx_act_app_date
    ON activity_sessions(app_id, started_at);

-- ----------------------------------------------------------
-- productivity_metrics: pre-computed daily summaries
-- Updated by the app after each session is written.
-- ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS productivity_metrics (
    date           TEXT PRIMARY KEY,   -- YYYY-MM-DD UTC
    total_ms       INTEGER NOT NULL DEFAULT 0,
    productive_ms  INTEGER NOT NULL DEFAULT 0,
    distraction_ms INTEGER NOT NULL DEFAULT 0,
    neutral_ms     INTEGER NOT NULL DEFAULT 0,
    focus_score    REAL    NOT NULL DEFAULT 0.0,
    session_count  INTEGER NOT NULL DEFAULT 0,
    switch_count   INTEGER NOT NULL DEFAULT 0,
    updated_at     TEXT    NOT NULL
);

-- ----------------------------------------------------------
-- settings: key-value store (JSON-encoded values)
-- ----------------------------------------------------------
CREATE TABLE IF NOT EXISTS settings (
    key        TEXT PRIMARY KEY,
    value      TEXT NOT NULL,
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);

-- ----------------------------------------------------------
-- Migrate legacy data from app_sessions (best-effort)
-- ended_at is set equal to started_at for migrated rows;
-- duration_ms remains the authoritative duration field.
-- ----------------------------------------------------------
INSERT OR IGNORE INTO apps
    (exe, display_name, exe_path, category_id, first_seen, last_seen, total_ms)
SELECT
    exe,
    exe,
    '',
    CASE category
        WHEN 'Productive'  THEN 1
        WHEN 'Distraction' THEN 3
        ELSE                    2
    END,
    MIN(started_at),
    MAX(started_at),
    SUM(duration_ms)
FROM app_sessions
GROUP BY exe;

INSERT OR IGNORE INTO activity_sessions (app_id, title, started_at, ended_at, duration_ms)
SELECT a.id, s.title, s.started_at, s.started_at, s.duration_ms
FROM   app_sessions s
JOIN   apps a ON a.exe = s.exe;
