-- Browser tab tracking
CREATE TABLE IF NOT EXISTS browser_events (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at  TEXT    NOT NULL,
    ended_at    TEXT,
    duration_ms INTEGER NOT NULL DEFAULT 0,
    browser     TEXT    NOT NULL,
    url         TEXT    NOT NULL,
    domain      TEXT    NOT NULL,
    title       TEXT    NOT NULL,
    category    TEXT    NOT NULL DEFAULT 'neutral',
    subcategory TEXT    NOT NULL DEFAULT 'general'
);

CREATE INDEX IF NOT EXISTS idx_browser_events_started ON browser_events(started_at);
CREATE INDEX IF NOT EXISTS idx_browser_events_domain  ON browser_events(domain);

-- User-defined domain → category overrides
CREATE TABLE IF NOT EXISTS browser_category_rules (
    id             INTEGER PRIMARY KEY AUTOINCREMENT,
    domain_pattern TEXT    NOT NULL UNIQUE,
    category       TEXT    NOT NULL CHECK (category IN ('productive','neutral','distraction')),
    subcategory    TEXT    NOT NULL DEFAULT 'custom'
);
