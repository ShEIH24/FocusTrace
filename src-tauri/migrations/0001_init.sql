CREATE TABLE IF NOT EXISTS app_sessions (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  exe         TEXT NOT NULL,
  title       TEXT NOT NULL,
  started_at  TEXT NOT NULL,
  duration_ms INTEGER NOT NULL,
  category    TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS app_rules (
  id       INTEGER PRIMARY KEY AUTOINCREMENT,
  pattern  TEXT NOT NULL UNIQUE,
  category TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_sessions_exe_date
  ON app_sessions(exe, started_at);

CREATE INDEX IF NOT EXISTS idx_sessions_date
  ON app_sessions(started_at);
