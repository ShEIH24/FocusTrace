import { create } from "zustand";
import type { PomodoroPhase } from "@/types/productivity";

interface PomodoroState {
  phase: PomodoroPhase;
  remainingSecs: number;
  totalSecs: number;
  sessionCount: number;
  isPaused: boolean;
  applyTick: (payload: {
    phase: PomodoroPhase;
    remainingSecs: number;
    totalSecs: number;
    sessionCount: number;
    isPaused: boolean;
  }) => void;
  applyPhaseChange: (payload: {
    phase: PomodoroPhase;
    sessionCount: number;
    totalSecs: number;
  }) => void;
  reset: () => void;
}

export const usePomodoroStore = create<PomodoroState>((set) => ({
  phase: "idle",
  remainingSecs: 0,
  totalSecs: 0,
  sessionCount: 0,
  isPaused: false,

  applyTick: (p) =>
    set({
      phase: p.phase,
      remainingSecs: p.remainingSecs,
      totalSecs: p.totalSecs,
      sessionCount: p.sessionCount,
      isPaused: p.isPaused,
    }),

  applyPhaseChange: (p) =>
    set({
      phase: p.phase,
      sessionCount: p.sessionCount,
      totalSecs: p.totalSecs,
      remainingSecs: p.totalSecs,
      isPaused: false,
    }),

  reset: () =>
    set({ phase: "idle", remainingSecs: 0, totalSecs: 0, isPaused: false }),
}));
