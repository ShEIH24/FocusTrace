// хук для получения переводов на текущем языке
import { useUiStore } from "@/store/uiStore";
import { translations } from "@/i18n/translations";

export function useT() {
  const language = useUiStore((s) => s.language);
  return translations[language];
}
