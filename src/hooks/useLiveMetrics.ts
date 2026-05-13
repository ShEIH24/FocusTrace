import { useEffect } from "react";
import { useQuery } from "@tanstack/react-query";

import { api } from "@/ipc/commands";
import { useMetricsStore } from "@/store/metricsStore";
import { useUiStore } from "@/store/uiStore";
import type { LiveCacheSnapshot } from "@/types/events";

/**
 * Returns live metrics for `selectedDate`, merging DB query data with
 * real-time updates from the `metrics-updated` event stream.
 *
 * Priority:
 *  - If `metricsStore.live` has data for today and is more recent than
 *    the last DB query, use it (zero-latency after a session ends).
 *  - Otherwise fall back to the `get_focus_score` + `get_app_usage` queries.
 */
export function useLiveMetrics() {
  const selectedDate = useUiStore((s) => s.selectedDate);
  const todayStr = new Date().toISOString().slice(0, 10);
  const isToday = selectedDate === todayStr;

  const live = useMetricsStore((s) => s.live);
  const liveIsForToday = live.date === todayStr && live.persisted;

  // Seed the metrics store from the backend cache on mount (avoids a gap
  // between app launch and the first `metrics-updated` event).
  const { data: cacheSnapshot } = useQuery<LiveCacheSnapshot>({
    queryKey: ["liveCache"],
    queryFn: () => api.getLiveCache(),
    staleTime: Infinity,   // only fetched once at startup
    refetchOnWindowFocus: false,
  });

  const applyMetricsUpdate = useMetricsStore((s) => s.applyMetricsUpdate);
  useEffect(() => {
    if (cacheSnapshot && isToday && !liveIsForToday) {
      applyMetricsUpdate({
        date: todayStr,
        focusScore: cacheSnapshot.todayScore,
        totalMs: cacheSnapshot.todayTotalMs,
        productiveMs: cacheSnapshot.todayProductiveMs,
        distractionMs: 0,
        neutralMs: 0,
        sessionCount: 0,
      });
    }
  }, [cacheSnapshot, isToday, liveIsForToday, todayStr, applyMetricsUpdate]);

  // DB-backed focus score (authoritative).
  const scoreQuery = useQuery<number, Error>({
    queryKey: ["focusScore", selectedDate],
    queryFn: () => api.getFocusScore(selectedDate),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });

  // Decide what to surface: prefer live store for today if fresh.
  const focusScore =
    isToday && liveIsForToday
      ? live.focusScore
      : (scoreQuery.data ?? (isToday && cacheSnapshot ? cacheSnapshot.todayScore : 0));

  const totalMs =
    isToday && liveIsForToday
      ? live.totalMs
      : (isToday && cacheSnapshot ? cacheSnapshot.todayTotalMs : 0);

  const productiveMs =
    isToday && liveIsForToday
      ? live.productiveMs
      : (isToday && cacheSnapshot ? cacheSnapshot.todayProductiveMs : 0);

  return {
    focusScore,
    totalMs,
    productiveMs,
    isLoading: scoreQuery.isLoading,
    isError: scoreQuery.isError,
    isLive: isToday && liveIsForToday,
  };
}

/** Hook for the hourly heatmap — backed by `get_hourly_stats`. */
export function useHourlyStats(date: string) {
  return useQuery({
    queryKey: ["hourlyStats", date],
    queryFn: () => api.getHourlyStats(date),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });
}

/** Full day report hook backed by `get_day_report`. */
export function useDayReport(date: string) {
  return useQuery({
    queryKey: ["dayReport", date],
    queryFn: () => api.getDayReport(date),
    staleTime: 60_000,
    refetchOnWindowFocus: false,
  });
}

/** 7-day week report hook backed by `get_week_report`. */
export function useWeekReport(endDate: string) {
  return useQuery({
    queryKey: ["weekReport", endDate],
    queryFn: () => api.getWeekReport(endDate),
    staleTime: 60_000,
    refetchOnWindowFocus: false,
  });
}
