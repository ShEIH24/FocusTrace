export type Category = "Productive" | "Neutral" | "Distraction";

export interface WindowInfo {
  exe: string;
  title: string;
  pid: number;
}

export interface SessionInfo {
  exe: string;
  title: string;
  started_at: string; // ISO 8601 UTC
  duration_ms: number;
  category: Category;
}

export interface AppUsageStat {
  exe: string;
  total_ms: number;
  category: Category;
  session_count: number;
}

export interface ActivityEvent {
  type: "window_changed" | "idle_started" | "idle_ended";
  payload: WindowInfo | { at: string } | { duration_secs: number };
}
