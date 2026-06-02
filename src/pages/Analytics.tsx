import { useQuery } from "@tanstack/react-query";
import { api } from "@/ipc/commands";
import { Card, CardHeader } from "@/components/ui/Card";
import { StatCard } from "@/components/ui/StatCard";
import { BarChart } from "@/components/charts/BarChart";
import { LineChart } from "@/components/charts/LineChart";
import { WeekHeatmap } from "@/components/charts/Heatmap";
import { useT } from "@/hooks/useT";
import type { WeeklySummary } from "@/types/analytics";
import { TbCalendarStats, TbTrendingUp, TbClock, TbTargetArrow } from "react-icons/tb";

function fmtMs(ms: number): string {
  const h = Math.floor(ms / 3_600_000);
  const m = Math.floor((ms % 3_600_000) / 60_000);
  return h > 0 ? `${h}h ${m}m` : `${m}m`;
}

function bestDay(days: string[], scores: number[], locale: string): string {
  if (days.length === 0) return "—";
  const idx = scores.indexOf(Math.max(...scores));
  const d = new Date(days[idx]);
  return d.toLocaleDateString(locale, { weekday: "short", month: "short", day: "numeric" });
}

export default function Analytics() {
  const t = useT();

  const { data, isLoading, isError } = useQuery<WeeklySummary, Error>({
    queryKey: ["weeklySummary"],
    queryFn: () => api.getWeeklySummary(),
    staleTime: 60_000,
    refetchOnWindowFocus: false,
  });

  const bars = data
    ? data.days.map((day, i) => ({
        label: new Date(day).toLocaleDateString(t.locale, { weekday: "short" }).slice(0, 2),
        value: Math.round(data.scores[i] ?? 0),
      }))
    : [];

  const linePoints = data
    ? data.days.map((day, i) => ({
        label: new Date(day).toLocaleDateString(t.locale, { weekday: "short" }).slice(0, 2),
        value: data.scores[i] ?? 0,
      }))
    : [];

  const heatmapData = data
    ? data.days.map((day, i) => ({ date: day, score: data.scores[i] ?? 0 }))
    : [];

  return (
    <div className="p-6 space-y-5 max-w-4xl mx-auto animate-fade-in">
      <div>
        <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">{t.analyticsTitle}</h1>
        <p className="text-sm text-muted mt-0.5">{t.sevenDayOverview}</p>
      </div>

      {isLoading && (
        <div className="flex items-center justify-center py-16">
          <p className="text-sm text-muted">{t.loadingAnalytics}</p>
        </div>
      )}
      {isError && <p className="text-sm text-red-500">{t.failedToLoadAnalytics}</p>}

      {data && (
        <>
          {/* строка статистики */}
          <div className="grid grid-cols-4 gap-3">
            <StatCard
              label={t.avgFocusScore}
              value={Math.round(data.avg_score)}
              sub={t.last7Days}
              icon={<TbTargetArrow size={16} />}
              accent="indigo"
            />
            <StatCard
              label={t.totalActive}
              value={fmtMs(data.total_active_ms)}
              icon={<TbClock size={16} />}
              accent="green"
            />
            <StatCard
              label={t.bestDay}
              value={bestDay(data.days, data.scores, t.locale)}
              icon={<TbTrendingUp size={16} />}
              accent="amber"
            />
            <StatCard
              label={t.daysTracked}
              value={data.days.filter((_, i) => (data.totalMsPerDay?.[i] ?? 0) > 0).length}
              sub={`${t.of} ${data.days.length}`}
              icon={<TbCalendarStats size={16} />}
              accent="indigo"
            />
          </div>

          {/* линейный и столбчатый графики */}
          <div className="grid grid-cols-2 gap-4">
            <Card>
              <CardHeader title={t.focusTrend} subtitle={t.scoreOverTime} />
              <LineChart points={linePoints} height={110} color="#6366f1" />
            </Card>

            <Card>
              <CardHeader title={t.dailyScores} subtitle={t.barChartView} />
              <BarChart bars={bars} height={110} />
            </Card>
          </div>

          {/* тепловая карта */}
          <Card>
            <CardHeader title={t.activityHeatmap} subtitle={t.focusScoreByDay} />
            <div className="flex items-center gap-4">
              <WeekHeatmap data={heatmapData} locale={t.locale} />
              <div className="flex flex-col gap-1.5 ml-2">
                {[
                  { color: "rgba(239,68,68,0.25)",  label: t.heatmapLow },
                  { color: "rgba(245,158,11,0.35)", label: t.heatmapMid },
                  { color: "rgba(34,197,94,0.45)",  label: t.heatmapHigh },
                ].map((l) => (
                  <div key={l.label} className="flex items-center gap-2">
                    <span className="w-3 h-3 rounded-sm border border-zinc-200 dark:border-zinc-700" style={{ background: l.color }} />
                    <span className="text-[10px] text-muted">{l.label}</span>
                  </div>
                ))}
              </div>
            </div>
          </Card>

          {/* распределение оценок по дням */}
          <Card>
            <CardHeader title={t.scoreDistribution} />
            <div className="grid grid-cols-7 gap-2">
              {data.days.map((day, i) => {
                const score = data.scores[i] ?? 0;
                const color = score < 40 ? "#ef4444" : score < 70 ? "#f59e0b" : "#22c55e";
                return (
                  <div key={day} className="flex flex-col items-center gap-1.5">
                    <div
                      className="w-full rounded-lg flex items-end justify-center text-[10px] font-bold text-white transition-all"
                      style={{
                        height: 60,
                        background: `${color}20`,
                        border: `1px solid ${color}40`,
                        position: "relative",
                      }}
                    >
                      <div
                        className="absolute bottom-0 left-0 right-0 rounded-lg transition-all"
                        style={{ height: `${score}%`, background: `${color}50` }}
                      />
                      <span className="relative z-10 pb-1" style={{ color }}>{Math.round(score)}</span>
                    </div>
                    <span className="text-[10px] text-muted">
                      {new Date(day).toLocaleDateString(t.locale, { weekday: "short" }).slice(0, 2)}
                    </span>
                  </div>
                );
              })}
            </div>
          </Card>
        </>
      )}
    </div>
  );
}
