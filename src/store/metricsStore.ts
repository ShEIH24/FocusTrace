import { create } from "zustand";
import type { MetricsUpdatedPayload } from "@/types/events";

interface LiveMetrics {
  /** ISO date string this snapshot is for, e.g. "2025-05-11". */
  date: string | null;
  focusScore: number;
  totalMs: number;
  productiveMs: number;
  distractionMs: number;
  neutralMs: number;
  sessionCount: number;
  /** Whether the metrics reflect data that has been persisted to the DB. */
  persisted: boolean;
  /** Wall-clock time of the last update (ms since epoch). */
  updatedAt: number;
}

interface MetricsState {
  live: LiveMetrics;
  /** Applies a `metrics-updated` event payload from the backend. */
  applyMetricsUpdate: (payload: MetricsUpdatedPayload) => void;
  /** Resets live metrics (e.g. on date change). */
  reset: () => void;
}

const DEFAULT_METRICS: LiveMetrics = {
  date: null,
  focusScore: 0,
  totalMs: 0,
  productiveMs: 0,
  distractionMs: 0,
  neutralMs: 0,
  sessionCount: 0,
  persisted: false,
  updatedAt: 0,
};

export const useMetricsStore = create<MetricsState>((set) => ({
  live: { ...DEFAULT_METRICS },

  applyMetricsUpdate: (payload) =>
    set({
      live: {
        date: payload.date,
        focusScore: payload.focusScore,
        totalMs: payload.totalMs,
        productiveMs: payload.productiveMs,
        distractionMs: payload.distractionMs,
        neutralMs: payload.neutralMs,
        sessionCount: payload.sessionCount,
        persisted: true,
        updatedAt: Date.now(),
      },
    }),

  reset: () => set({ live: { ...DEFAULT_METRICS } }),
}));
