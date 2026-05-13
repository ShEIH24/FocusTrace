-- ============================================================
-- Migration 0003: Productivity features
-- Pomodoro sessions log + daily goal snapshots
-- ============================================================

CREATE TABLE IF NOT EXISTS pomodoro_sessions (
    id           INTEGER PRIMARY KEY AUTOINCREMENT,
    started_at   TEXT    NOT NULL,
    ended_at     TEXT,                     -- NULL while in progress
    duration_ms  INTEGER NOT NULL DEFAULT 0,
    phase        TEXT    NOT NULL
                         CHECK (phase IN ('work', 'short_break', 'long_break')),
    completed    INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
    interrupted  INTEGER NOT NULL DEFAULT 0 CHECK (interrupted IN (0, 1))
);

CREATE INDEX IF NOT EXISTS idx_pomo_started ON pomodoro_sessions(started_at);

-- One snapshot per calendar day recording goal attainment.
-- Written each time get_daily_goals is called on a new day.
CREATE TABLE IF NOT EXISTS daily_goal_results (
    date                 TEXT    PRIMARY KEY,
    productive_target_ms INTEGER NOT NULL DEFAULT 0,
    pomodoro_target      INTEGER NOT NULL DEFAULT 0,
    score_target         REAL    NOT NULL DEFAULT 0.0,
    productive_actual_ms INTEGER NOT NULL DEFAULT 0,
    pomodoro_actual      INTEGER NOT NULL DEFAULT 0,
    score_actual         REAL    NOT NULL DEFAULT 0.0,
    goal_met             INTEGER NOT NULL DEFAULT 0 CHECK (goal_met IN (0, 1)),
    updated_at           TEXT    NOT NULL
                         DEFAULT (strftime('%Y-%m-%dT%H:%M:%SZ', 'now'))
);
