import { TbArrowUpCircle, TbX } from "react-icons/tb";
import { open } from "@tauri-apps/plugin-shell";
import { useUiStore } from "@/store/uiStore";
import { useT } from "@/hooks/useT";

export default function UpdateBanner() {
  const pendingUpdateVersion = useUiStore((s) => s.pendingUpdateVersion);
  const setPendingUpdateVersion = useUiStore((s) => s.setPendingUpdateVersion);
  const t = useT();

  if (!pendingUpdateVersion) return null;

  return (
    <div className="flex items-center gap-3 px-4 py-3 rounded-xl
      bg-indigo-50 dark:bg-indigo-500/5 border border-indigo-200 dark:border-indigo-500/20
      animate-fade-in">
      <TbArrowUpCircle size={18} className="text-indigo-500 flex-shrink-0" />
      <div className="flex-1 min-w-0">
        <p className="text-sm font-semibold text-indigo-800 dark:text-indigo-300">
          {t.updateAvailable}
        </p>
        <p className="text-xs text-indigo-600 dark:text-indigo-400/80">
          {t.updateVersion} {pendingUpdateVersion}
        </p>
      </div>
      <button
        onClick={() => open("https://github.com/ShEIH24/FocusTrace/releases/latest")}
        className="shrink-0 flex items-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-medium
          bg-indigo-600 hover:bg-indigo-500 text-white transition-colors"
      >
        {t.installUpdate}
      </button>
      <button
        onClick={() => setPendingUpdateVersion(null)}
        className="shrink-0 p-1 rounded-lg text-indigo-400 hover:bg-indigo-100 dark:hover:bg-indigo-900/30 transition-colors"
        aria-label="Dismiss"
      >
        <TbX size={16} />
      </button>
    </div>
  );
}
