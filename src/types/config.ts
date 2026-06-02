export interface AppConfig {
  idle_threshold_secs: number;
  poll_interval_ms: number;
  data_dir: string;
  productive_apps: string[];
  distraction_apps: string[];
  excluded_apps: string[];
}

export interface AppRule {
  id: number;
  pattern: string;
  category: "Productive" | "Neutral" | "Distraction";
}

export type IpcPayload<T> =
  | { data: T }
  | { error: { code: string; message: string } };
