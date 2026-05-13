/** Full per-day analytics report returned by `get_day_report`. */
export interface DayReport {
  date: string;
  totalMs: number;
  productiveMs: number;
  distractionMs: number;
  neutralMs: number;
  sessionCount: number;
  score: ScoreBreakdown;
  focusSessions: FocusSession[];
  deepWork: DeepWorkInterval[];
  contextSwitches: ContextSwitchStats;
  appBreakdown: AppBreakdown[];
  categoryBreakdown: CategoryBreakdown[];
  summary: string;
}

export interface WeekReport {
  startDate: string;
  endDate: string;
  days: DayReport[];
  avgScore: number;
  totalMs: number;
  productiveMs: number;
  topApps: AppBreakdown[];
  summary: string;
}

export interface ScoreBreakdown {
  total: number;
  productiveRatioComponent: number;
  focusDepthComponent: number;
  switchPenaltyComponent: number;
  continuityComponent: number;
}

export type DepthLevel = "Shallow" | "Focused" | "Deep" | "Flow";

export interface FocusSession {
  startedAt: string;
  endedAt: string;
  durationMs: number;
  depth: DepthLevel;
  primaryApp: string;
  sessionCount: number;
}

export interface DeepWorkInterval {
  startedAt: string;
  endedAt: string;
  durationMs: number;
  depth: DepthLevel;
  apps: string[];
}

export interface ContextSwitchStats {
  totalSwitches: number;
  switchesPerHour: number;
  rapidSwitches: number;
  sprees: SwitchSpree[];
  insight: string;
}

export interface SwitchSpree {
  startedAt: string;
  endedAt: string;
  switchCount: number;
}

export interface AppBreakdown {
  exe: string;
  displayName: string;
  category: string;
  totalMs: number;
  sessionCount: number;
  percentage: number;
  formatted: string;
}

export interface CategoryBreakdown {
  category: string;
  totalMs: number;
  percentage: number;
  formatted: string;
}

export interface HourlyStats {
  hour: number;
  totalMs: number;
  productiveMs: number;
  sessionCount: number;
}
