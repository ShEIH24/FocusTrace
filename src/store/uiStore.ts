import { create } from "zustand";
import type { Language } from "@/i18n/translations";

type Theme = "light" | "dark" | "system";

interface UiState {
  selectedDate: string;
  theme: Theme;
  language: Language;
  pendingUpdateVersion: string | null;
  setDate: (date: string) => void;
  setTheme: (theme: Theme) => void;
  setLanguage: (lang: Language) => void;
  setPendingUpdateVersion: (v: string | null) => void;
}

function todayIso(): string {
  return new Date().toISOString().slice(0, 10);
}

// читаем язык из localStorage, чтобы он сохранялся между запусками
function savedLanguage(): Language {
  const v = localStorage.getItem("language");
  return v === "ru" ? "ru" : "en";
}

export const useUiStore = create<UiState>((set) => ({
  selectedDate: todayIso(),
  theme: "system",
  language: savedLanguage(),
  pendingUpdateVersion: null,
  setDate: (date) => set({ selectedDate: date }),
  setTheme: (theme) => set({ theme }),
  setLanguage: (language) => {
    localStorage.setItem("language", language);
    set({ language });
  },
  setPendingUpdateVersion: (v) => set({ pendingUpdateVersion: v }),
}));
