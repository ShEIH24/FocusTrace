import { useEffect, useState } from "react";
import { useActivityStore } from "@/store/activityStore";
import { CategoryBadge } from "@/components/ui/Badge";
import { TbActivity } from "react-icons/tb";
import { useT } from "@/hooks/useT";
import type { Category } from "@/types/activity";

function useElapsedTimer(): string {
  const [elapsed, setElapsed] = useState(0);
  useEffect(() => {
    setElapsed(0);
    const id = setInterval(() => setElapsed((e) => e + 1), 1000);
    return () => clearInterval(id);
  }, []);
  const m = Math.floor(elapsed / 60);
  const s = elapsed % 60;
  return `${m}:${String(s).padStart(2, "0")}`;
}

export default function ActivityCard() {
  const currentWindow = useActivityStore((s) => s.currentWindow);
  const elapsed = useElapsedTimer();
  const t = useT();

  if (!currentWindow) {
    return (
      <div className="card p-4 flex items-center gap-3">
        <div className="w-8 h-8 rounded-lg bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center">
          <TbActivity size={16} className="text-zinc-400" />
        </div>
        <p className="text-sm text-muted">{t.noActiveWindow}</p>
      </div>
    );
  }

  const category = (currentWindow as { category?: Category }).category ?? "Neutral";
  const colorMap: Record<Category, string> = {
    Productive: "bg-green-500",
    Neutral:    "bg-zinc-400",
    Distraction: "bg-red-500",
  };

  return (
    <div className="card p-4 flex items-center gap-4 animate-fade-in">
      <div className="relative flex-shrink-0">
        <div className="w-9 h-9 rounded-xl bg-zinc-100 dark:bg-zinc-800 flex items-center justify-center">
          <TbActivity size={18} className="text-zinc-600 dark:text-zinc-300" />
        </div>
        <span className={`absolute -bottom-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-white dark:border-zinc-900 ${colorMap[category]} animate-pulse2`} />
      </div>
      <div className="flex-1 min-w-0">
        <p className="text-label mb-0.5">{t.activeNow}</p>
        <p className="text-sm font-semibold text-zinc-900 dark:text-zinc-100 truncate">{currentWindow.exe}</p>
        <p className="text-xs text-muted truncate mt-0.5">{currentWindow.title || "—"}</p>
      </div>
      <div className="flex flex-col items-end gap-1 flex-shrink-0">
        <CategoryBadge category={category} />
        <p className="text-base font-mono font-bold text-indigo-500 dark:text-indigo-400">{elapsed}</p>
      </div>
    </div>
  );
}
