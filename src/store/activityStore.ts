import { create } from "zustand";
import type { WindowInfo } from "@/types/activity";

interface ActivityState {
  currentWindow: WindowInfo | null;
  isIdle: boolean;
  idleSince: Date | null;
  setCurrentWindow: (w: WindowInfo | null) => void;
  setIdleStatus: (isIdle: boolean, since?: Date) => void;
}

export const useActivityStore = create<ActivityState>((set) => ({
  currentWindow: null,
  isIdle: false,
  idleSince: null,
  setCurrentWindow: (w) => set({ currentWindow: w }),
  setIdleStatus: (isIdle, since) =>
    set({ isIdle, idleSince: since ?? null }),
}));
