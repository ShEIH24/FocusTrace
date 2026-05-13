import { invoke } from "@tauri-apps/api/core";
import type { AppConfig } from "@/types/config";
import type { AppUsageStat, SessionInfo, WindowInfo } from "@/types/activity";
import type { WeeklySummary } from "@/types/analytics";
import type { DayReport, HourlyStats, WeekReport } from "@/types/reports";
import type { LiveCacheSnapshot } from "@/types/events";
import type { GoalProgress, PomodoroTickPayload, ProductivityConfig, StreakInfo } from "@/types/productivity";
import type { BrowserEventRow, DomainStat, TabInfo, UserCategoryRule } from "@/types/browser";

export const api = {
  // ---------------------------------------------------------------------------
  // Activity
  // ---------------------------------------------------------------------------

  getCurrentActivity: (): Promise<WindowInfo | null> =>
    invoke<WindowInfo | null>("get_current_activity"),

  getActivityLog: (date: string): Promise<SessionInfo[]> =>
    invoke<SessionInfo[]>("get_activity_log", { date }),

  getAppUsage: (date: string): Promise<AppUsageStat[]> =>
    invoke<AppUsageStat[]>("get_app_usage", { date }),

  /** Hourly breakdown for a given date — sparse, hours with no activity omitted. */
  getHourlyStats: (date: string): Promise<HourlyStats[]> =>
    invoke<HourlyStats[]>("get_hourly_stats", { date }),

  /** In-memory cache snapshot — use on startup to avoid waiting for the first event. */
  getLiveCache: (): Promise<LiveCacheSnapshot> =>
    invoke<LiveCacheSnapshot>("get_live_cache"),

  // ---------------------------------------------------------------------------
  // Analytics
  // ---------------------------------------------------------------------------

  getFocusScore: (date: string): Promise<number> =>
    invoke<number>("get_focus_score", { date }),

  getWeeklySummary: (): Promise<WeeklySummary> =>
    invoke<WeeklySummary>("get_weekly_summary"),

  /** Full per-day analytics report including focus sessions, deep work, context switches. */
  getDayReport: (date: string): Promise<DayReport> =>
    invoke<DayReport>("get_day_report", { date }),

  /** 7-day aggregate ending on `endDate`. */
  getWeekReport: (endDate: string): Promise<WeekReport> =>
    invoke<WeekReport>("get_week_report", { endDate }),

  // ---------------------------------------------------------------------------
  // Settings
  // ---------------------------------------------------------------------------

  getConfig: (): Promise<AppConfig> => invoke<AppConfig>("get_config"),

  updateConfig: (patch: Partial<AppConfig>): Promise<void> =>
    invoke<void>("update_config", { patch }),

  // ---------------------------------------------------------------------------
  // Productivity — Pomodoro
  // ---------------------------------------------------------------------------

  pomodoroStart: (): Promise<void> => invoke<void>("pomodoro_start"),

  pomodoroPause: (): Promise<void> => invoke<void>("pomodoro_pause"),

  pomodoroResume: (): Promise<void> => invoke<void>("pomodoro_resume"),

  pomodoroStop: (): Promise<void> => invoke<void>("pomodoro_stop"),

  pomodoroSkip: (): Promise<void> => invoke<void>("pomodoro_skip"),

  getPomodoroState: (): Promise<PomodoroTickPayload> =>
    invoke<PomodoroTickPayload>("get_pomodoro_state"),

  getProductivityConfig: (): Promise<ProductivityConfig> =>
    invoke<ProductivityConfig>("get_productivity_config"),

  updateProductivityConfig: (config: ProductivityConfig): Promise<void> =>
    invoke<void>("update_productivity_config", { config }),

  // ---------------------------------------------------------------------------
  // Productivity — Goals & streaks
  // ---------------------------------------------------------------------------

  getDailyGoals: (): Promise<GoalProgress> => invoke<GoalProgress>("get_daily_goals"),

  getStreak: (): Promise<StreakInfo> => invoke<StreakInfo>("get_streak"),

  // ---------------------------------------------------------------------------
  // Productivity — Focus mode
  // ---------------------------------------------------------------------------

  toggleFocusMode: (): Promise<boolean> => invoke<boolean>("toggle_focus_mode"),

  getFocusModeState: (): Promise<boolean> => invoke<boolean>("get_focus_mode_state"),

  // ---------------------------------------------------------------------------
  // Browser tracking
  // ---------------------------------------------------------------------------

  getBrowserWsToken: (): Promise<string> =>
    invoke<string>("get_browser_ws_token"),

  getCurrentBrowserTab: (): Promise<TabInfo | null> =>
    invoke<TabInfo | null>("get_current_browser_tab"),

  getBrowserHistory: (date: string): Promise<BrowserEventRow[]> =>
    invoke<BrowserEventRow[]>("get_browser_history", { date }),

  getBrowserDomainStats: (date: string): Promise<DomainStat[]> =>
    invoke<DomainStat[]>("get_browser_domain_stats", { date }),

  getBrowserTodaySummary: (): Promise<[string, number][]> =>
    invoke<[string, number][]>("get_browser_today_summary"),

  addBrowserCategoryRule: (
    domainPattern: string,
    category: string,
    subcategory: string,
  ): Promise<void> =>
    invoke<void>("add_browser_category_rule", { domainPattern, category, subcategory }),

  removeBrowserCategoryRule: (domainPattern: string): Promise<void> =>
    invoke<void>("remove_browser_category_rule", { domainPattern }),

  getBrowserCategoryRules: (): Promise<UserCategoryRule[]> =>
    invoke<UserCategoryRule[]>("get_browser_category_rules"),

  categorizeUrl: (url: string): Promise<[string, string]> =>
    invoke<[string, string]>("categorize_url", { url }),

  // ---------------------------------------------------------------------------
  // Window / tray
  // ---------------------------------------------------------------------------

  showMainWindow: (): Promise<void> => invoke<void>("show_main_window"),

  hideMainWindow: (): Promise<void> => invoke<void>("hide_main_window"),
};
