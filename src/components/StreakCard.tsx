import { useQuery } from "@tanstack/react-query";
import { api } from "@/ipc/commands";
import { useT } from "@/hooks/useT";
import type { StreakInfo } from "@/types/productivity";

export default function StreakCard() {
  const t = useT();
  const { data } = useQuery<StreakInfo>({
    queryKey: ["streak"],
    queryFn: () => api.getStreak(),
    refetchInterval: 5 * 60_000,
  });

  if (!data) {
    return <div className="h-24 rounded-xl bg-zinc-800 animate-pulse" />;
  }

  return (
    <div className="flex flex-col gap-4">
      {/* счётчики серий */}
      <div className="flex gap-6">
        <div className="flex flex-col gap-0.5">
          <span className="text-2xl font-bold text-zinc-100">{data.current}</span>
          <span className="text-xs text-zinc-500">{t.currentStreak}</span>
        </div>
        <div className="flex flex-col gap-0.5">
          <span className="text-2xl font-bold text-zinc-100">{data.longest}</span>
          <span className="text-xs text-zinc-500">{t.longestStreak}</span>
        </div>
      </div>

      {/* точки за последние 7 дней */}
      <div className="flex gap-2">
        {data.last7Days.map((productive, i) => {
          const dayIndex = (new Date().getDay() - 6 + i + 7) % 7;
          return (
            <div key={i} className="flex flex-col items-center gap-1">
              <div
                className={`w-7 h-7 rounded-lg flex items-center justify-center text-xs font-medium transition-colors ${
                  productive
                    ? "bg-indigo-600 text-white"
                    : "bg-zinc-800 text-zinc-600"
                }`}
              >
                {productive ? "✓" : ""}
              </div>
              <span className="text-[10px] text-zinc-600">{t.dayLabels[dayIndex]}</span>
            </div>
          );
        })}
      </div>

      {data.lastProductiveDate && (
        <p className="text-xs text-zinc-600">
          {t.lastProductive} {new Date(data.lastProductiveDate).toLocaleDateString(t.locale)}
        </p>
      )}
    </div>
  );
}
