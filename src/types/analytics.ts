export interface FocusScore {
  value: number; // 0-100
  date: string;
}

export interface WeeklySummary {
  days: string[];
  scores: number[];
  total_active_ms: number;
  avg_score: number;
}
