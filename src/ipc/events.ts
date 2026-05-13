import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type {
  ActivityUpdatedPayload,
  IdleChangedPayload,
  MetricsUpdatedPayload,
  SessionEndedPayload,
} from "@/types/events";
import type {
  AppNotificationPayload,
  DistractionAlertPayload,
  PomodoroPhaseChangedPayload,
  PomodoroTickPayload,
} from "@/types/productivity";
import type { TabInfo } from "@/types/browser";

// Activity
export const onActivityUpdated = (h: (p: ActivityUpdatedPayload) => void): Promise<UnlistenFn> =>
  listen<ActivityUpdatedPayload>("activity-updated", (e) => h(e.payload));

export const onIdleChanged = (h: (p: IdleChangedPayload) => void): Promise<UnlistenFn> =>
  listen<IdleChangedPayload>("idle-changed", (e) => h(e.payload));

export const onSessionEnded = (h: (p: SessionEndedPayload) => void): Promise<UnlistenFn> =>
  listen<SessionEndedPayload>("session-ended", (e) => h(e.payload));

export const onMetricsUpdated = (h: (p: MetricsUpdatedPayload) => void): Promise<UnlistenFn> =>
  listen<MetricsUpdatedPayload>("metrics-updated", (e) => h(e.payload));

// Productivity
export const onPomodoroTick = (h: (p: PomodoroTickPayload) => void): Promise<UnlistenFn> =>
  listen<PomodoroTickPayload>("pomodoro-tick", (e) => h(e.payload));

export const onPomodoroPhaseChanged = (h: (p: PomodoroPhaseChangedPayload) => void): Promise<UnlistenFn> =>
  listen<PomodoroPhaseChangedPayload>("pomodoro-phase-changed", (e) => h(e.payload));

export const onDistractionAlert = (h: (p: DistractionAlertPayload) => void): Promise<UnlistenFn> =>
  listen<DistractionAlertPayload>("distraction-alert", (e) => h(e.payload));

export const onAppNotification = (h: (p: AppNotificationPayload) => void): Promise<UnlistenFn> =>
  listen<AppNotificationPayload>("app-notification", (e) => h(e.payload));

// Browser
export const onBrowserTabUpdated = (h: (p: TabInfo) => void): Promise<UnlistenFn> =>
  listen<TabInfo>("browser-tab-updated", (e) => h(e.payload));
