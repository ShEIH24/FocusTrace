/** Payload of the `activity-updated` Tauri event. */
export interface ActivityUpdatedPayload {
  exe: string;
  title: string;
  pid: number;
  exePath: string;
}

/** Payload of the `idle-changed` Tauri event. */
export interface IdleChangedPayload {
  isIdle: boolean;
  /** ISO timestamp — present when isIdle becomes true */
  at?: string;
  /** Total idle duration in seconds — present when isIdle becomes false */
  durationSecs?: number;
}

/** Payload of the `session-ended` Tauri event.
 *  Fired immediately when a session completes, before the DB write. */
export interface SessionEndedPayload {
  exe: string;
  title: string;
  startedAt: string;
  durationMs: number;
  category: string;
}

/** Payload of the `metrics-updated` Tauri event.
 *  Fired after daily aggregates are recomputed and written to the DB cache. */
export interface MetricsUpdatedPayload {
  date: string;
  focusScore: number;
  totalMs: number;
  productiveMs: number;
  distractionMs: number;
  neutralMs: number;
  sessionCount: number;
}

/** Snapshot of the backend in-memory cache, returned by `get_live_cache`. */
export interface LiveCacheSnapshot {
  currentWindow: { exe: string; title: string; pid: number; exePath: string } | null;
  isIdle: boolean;
  idleSince: string | null;
  todayScore: number;
  todayTotalMs: number;
  todayProductiveMs: number;
}
