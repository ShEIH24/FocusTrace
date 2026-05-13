export type PomodoroPhase = "idle" | "working" | "short_break" | "long_break";

export interface PomodoroTickPayload {
  phase: PomodoroPhase;
  remainingSecs: number;
  totalSecs: number;
  sessionCount: number;
  isPaused: boolean;
}

export interface PomodoroPhaseChangedPayload {
  phase: PomodoroPhase;
  sessionCount: number;
  totalSecs: number;
}

export interface PomodoroConfig {
  workMins: number;
  shortBreakMins: number;
  longBreakMins: number;
  sessionsUntilLong: number;
  autoStartBreaks: boolean;
  autoStartWork: boolean;
}

export interface GoalConfig {
  productiveTargetMins: number;
  pomodoroTarget: number;
  scoreTarget: number;
}

export interface FocusModeConfig {
  enabled: boolean;
  alertThresholdSecs: number;
}

export interface ProductivityConfig {
  pomodoro: PomodoroConfig;
  goals: GoalConfig;
  focusMode: FocusModeConfig;
}

export interface GoalProgress {
  date: string;
  productiveTargetMs: number;
  productiveActualMs: number;
  productivePct: number;
  pomodoroTarget: number;
  pomodoroActual: number;
  pomodoroPct: number;
  scoreTarget: number;
  scoreActual: number;
  scorePct: number;
  overallMet: boolean;
}

export interface StreakInfo {
  current: number;
  longest: number;
  last7Days: boolean[];
  lastProductiveDate: string | null;
}

export interface DistractionAlertPayload {
  exe: string;
  distractionCount: number;
  activeSecs: number;
}

export interface AppNotificationPayload {
  title: string;
  body: string;
}
