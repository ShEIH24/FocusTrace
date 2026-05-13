import { create } from "zustand";
import type { AppConfig } from "@/types/config";
import { api } from "@/ipc/commands";

interface SettingsState {
  config: AppConfig | null;
  isLoading: boolean;
  setConfig: (config: AppConfig) => void;
  loadConfig: () => Promise<void>;
}

export const useSettingsStore = create<SettingsState>((set) => ({
  config: null,
  isLoading: false,
  setConfig: (config) => set({ config }),
  loadConfig: async () => {
    set({ isLoading: true });
    try {
      const config = await api.getConfig();
      set({ config, isLoading: false });
    } catch {
      set({ isLoading: false });
    }
  },
}));
