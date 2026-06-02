import { useCallback, useEffect } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "sonner";

import {
  onActivityUpdated,
  onAppNotification,
  onBrowserTabUpdated,
  onDistractionAlert,
  onIdleChanged,
  onMetricsUpdated,
  onPomodoroPhaseChanged,
  onPomodoroTick,
  onSessionEnded,
} from "@/ipc/events";
import { useActivityStore } from "@/store/activityStore";
import { useBrowserStore } from "@/store/browserStore";
import { useMetricsStore } from "@/store/metricsStore";
import { usePomodoroStore } from "@/store/pomodoroStore";
import type { ActivityUpdatedPayload } from "@/types/events";
import type { WindowInfo } from "@/types/activity";

/**
 * Mount once at the layout root.
 *
 * Wires all backend Tauri events to:
 *  1. Zustand stores   — immediate, zero-latency UI updates
 *  2. React Query       — query invalidation so the next render fetches fresh DB data
 *  3. Toasts            — app-notification and distraction-alert
 */
export function useRealtimeUpdates(): void {
  const queryClient = useQueryClient();
  const setCurrentWindow = useActivityStore((s) => s.setCurrentWindow);
  const setIdleStatus = useActivityStore((s) => s.setIdleStatus);
  const applyMetricsUpdate = useMetricsStore((s) => s.applyMetricsUpdate);
  const applyTick = usePomodoroStore((s) => s.applyTick);
  const applyPhaseChange = usePomodoroStore((s) => s.applyPhaseChange);
  const setCurrentTab = useBrowserStore((s) => s.setCurrentTab);

  const invalidateGoals = useCallback(() => {
    queryClient.invalidateQueries({ queryKey: ["dailyGoals"] });
    queryClient.invalidateQueries({ queryKey: ["streak"] });
  }, [queryClient]);

  // ── activity-updated ────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onActivityUpdated((payload: ActivityUpdatedPayload) => {
      const windowInfo: WindowInfo = {
        exe: payload.exe,
        title: payload.title,
        pid: payload.pid,
      };
      setCurrentWindow(windowInfo);
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [setCurrentWindow]);

  // ── idle-changed ────────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onIdleChanged((payload) => {
      if (payload.isIdle) {
        setIdleStatus(true, payload.at ? new Date(payload.at) : new Date());
      } else {
        setIdleStatus(false);
      }
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [setIdleStatus]);

  // ── session-ended ────────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onSessionEnded((payload) => {
      const date = payload.startedAt.slice(0, 10);
      queryClient.invalidateQueries({ queryKey: ["activity", date] });
      queryClient.invalidateQueries({ queryKey: ["appUsage", date] });
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [queryClient]);

  // ── metrics-updated ──────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onMetricsUpdated((payload) => {
      applyMetricsUpdate(payload);

      const date = payload.date;
      queryClient.invalidateQueries({ queryKey: ["focusScore", date] });
      queryClient.invalidateQueries({ queryKey: ["appUsage", date] });
      queryClient.invalidateQueries({ queryKey: ["activity", date] });
      queryClient.invalidateQueries({ queryKey: ["hourlyStats", date] });
      queryClient.invalidateQueries({ queryKey: ["dayReport", date] });
      queryClient.invalidateQueries({ queryKey: ["weeklySummary"] });
      queryClient.invalidateQueries({ queryKey: ["weekReport"] });
      invalidateGoals();
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [queryClient, applyMetricsUpdate, invalidateGoals]);

  // ── pomodoro-tick ────────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onPomodoroTick((payload) => {
      applyTick(payload);
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [applyTick]);

  // ── pomodoro-phase-changed ───────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onPomodoroPhaseChanged((payload) => {
      applyPhaseChange(payload);
      // Pomodoro completion may affect goal progress.
      if (payload.phase === "working") {
        invalidateGoals();
      }
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [applyPhaseChange, invalidateGoals]);

  // ── distraction-alert ────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onDistractionAlert((payload) => {
      toast.warning(`Focus alert: ${payload.exe}`, {
        description: `Distraction #${payload.distractionCount} — active for ${Math.round(payload.activeSecs / 60)}m`,
        duration: 6000,
      });
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, []);

  // ── app-notification ─────────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onAppNotification((payload) => {
      toast(payload.title, {
        description: payload.body,
        duration: 5000,
      });
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, []);

  // ── browser-tab-updated ──────────────────────────────────────────────────
  useEffect(() => {
    let cancel = false;
    let unlisten: (() => void) | undefined;

    onBrowserTabUpdated((tab) => {
      setCurrentTab(tab);
      // The previous tab's event was just saved to DB — invalidate today's browser history
      // so the Timeline reflects the new entry without requiring a manual refresh.
      const today = new Date().toISOString().slice(0, 10);
      queryClient.invalidateQueries({ queryKey: ["browserHistory", today] });
    }).then((fn) => {
      if (!cancel) unlisten = fn;
      else fn();
    });

    return () => {
      cancel = true;
      unlisten?.();
    };
  }, [setCurrentTab, queryClient]);
}
