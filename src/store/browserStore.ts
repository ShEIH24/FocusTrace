import { create } from "zustand";
import type { TabInfo } from "@/types/browser";

interface BrowserState {
  currentTab: TabInfo | null;
  setCurrentTab: (tab: TabInfo | null) => void;
}

export const useBrowserStore = create<BrowserState>((set) => ({
  currentTab: null,
  setCurrentTab: (tab) => set({ currentTab: tab }),
}));
