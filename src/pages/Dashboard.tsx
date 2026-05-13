import { useQuery } from "@tanstack/react-query";
import ActivityCard from "@/components/ActivityCard";
import IdleBanner from "@/components/IdleBanner";
import ScoreGauge from "@/components/ScoreGauge";
import { StatCard } from "@/components/ui/StatCard";
import { Card, CardHeader } from "@/components/ui/Card";
import { DonutChart, DonutLegend } from "@/components/charts/DonutChart";
import { api } from "@/ipc/commands";
import { useActivityStore } from "@/store/activityStore";
import { useUiStore } from "@/store/uiStore";
import { useTauriEvent } from "@/hooks/useTauriEvents";
import { useLiveMetrics } from "@/hooks/useLiveMetrics";
import { useT } from "@/hooks/useT";
import type { AppUsageStat, WindowInfo } from "@/types/activity";
import type { DomainStat } from "@/types/browser";
import { TbClock, TbFlame, TbBolt, TbTargetArrow, TbWorld, TbApps } from "react-icons/tb";

function fmtMs(ms: number): string {
  const m = Math.floor(ms / 60_000);
  if (m < 60) return `${m}m`;
  return `${Math.floor(m / 60)}h ${m % 60}m`;
}

function fmtMsShort(ms: number): string {
  const m = Math.floor(ms / 60_000);
  if (m < 60) return `${m}m`;
  const h = Math.floor(m / 60);
  const rem = m % 60;
  return rem > 0 ? `${h}h ${rem}m` : `${h}h`;
}

function categoryStats(stats: AppUsageStat[]) {
  let productive = 0, neutral = 0, distraction = 0;
  for (const s of stats) {
    if (s.category === "Productive") productive += s.total_ms;
    else if (s.category === "Neutral") neutral += s.total_ms;
    else distraction += s.total_ms;
  }
  const total = productive + neutral + distraction;
  const pct = total > 0 ? Math.round((productive / total) * 100) : 0;
  return { productive, neutral, distraction, total, pct };
}

export default function Dashboard() {
  const selectedDate = useUiStore((s) => s.selectedDate);
  const setCurrentWindow = useActivityStore((s) => s.setCurrentWindow);
  const t = useT();

  // activity-updated обрабатывается глобально в Layout, но здесь нужен для живого таймера в ActivityCard
  useTauriEvent<WindowInfo>("activity-updated", setCurrentWindow);

  // живые метрики — объединяет данные из БД и события metrics-updated
  const { focusScore, totalMs, productiveMs, isLoading: scoreLoading } = useLiveMetrics();

  const usageQuery = useQuery<AppUsageStat[], Error>({
    queryKey: ["appUsage", selectedDate],
    queryFn: () => api.getAppUsage(selectedDate),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });

  const browserQuery = useQuery<DomainStat[], Error>({
    queryKey: ["browserDomainStats", selectedDate],
    queryFn: () => api.getBrowserDomainStats(selectedDate),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });

  const stats = usageQuery.data ? categoryStats(usageQuery.data) : null;

  type CombinedItem = { key: string; name: string; total_ms: number; category: string; type: "app" | "site" };
  const combinedActivity: CombinedItem[] = [
    ...(usageQuery.data ?? []).map((a) => ({
      key: `app:${a.exe}`,
      name: a.exe,
      total_ms: a.total_ms,
      category: a.category.toLowerCase(),
      type: "app" as const,
    })),
    ...(browserQuery.data ?? []).map((d) => ({
      key: `site:${d.domain}`,
      name: d.domain,
      total_ms: d.durationMs,
      category: d.category.toLowerCase(),
      type: "site" as const,
    })),
  ].sort((a, b) => b.total_ms - a.total_ms).slice(0, 8);

  const maxActivityMs = combinedActivity[0]?.total_ms ?? 1;

  // предпочитаем живые данные из стора, если они есть
  const displayTotal = totalMs > 0 ? totalMs : (stats?.total ?? 0);
  const displayProductive = productiveMs > 0 ? productiveMs : (stats?.productive ?? 0);
  const displayPct = displayTotal > 0
    ? Math.round((displayProductive / displayTotal) * 100)
    : (stats?.pct ?? 0);

  const donutSlices = stats
    ? [
        { label: t.categoryProductive, value: stats.productive,  color: "#22c55e" },
        { label: t.categoryNeutral,    value: stats.neutral,     color: "#71717a" },
        { label: t.categoryDistraction, value: stats.distraction, color: "#ef4444" },
      ]
    : [];


  return (
    <div className="p-6 space-y-5 max-w-5xl mx-auto animate-fade-in">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">{t.dashboardTitle}</h1>
          <p className="text-sm text-muted mt-0.5">
            {new Date(selectedDate).toLocaleDateString(t.locale, {
              weekday: "long", month: "long", day: "numeric",
            })}
          </p>
        </div>
      </div>

      <IdleBanner />
      <ActivityCard />

      {/* статистика — живые значения обновляются сразу после завершения сессии */}
      <div className="grid grid-cols-4 gap-3">
        <StatCard
          label={t.totalActive}
          value={displayTotal > 0 ? fmtMsShort(displayTotal) : "—"}
          icon={<TbClock size={16} />}
          accent="indigo"
        />
        <StatCard
          label={t.focusScore}
          value={focusScore > 0 ? Math.round(focusScore) : scoreLoading ? "…" : "—"}
          sub={t.outOf100}
          icon={<TbTargetArrow size={16} />}
          accent="green"
        />
        <StatCard
          label={t.productive}
          value={displayTotal > 0 ? `${displayPct}%` : "—"}
          sub={displayProductive > 0 ? fmtMsShort(displayProductive) : undefined}
          icon={<TbFlame size={16} />}
          accent="green"
        />
        <StatCard
          label={t.sessions}
          value={usageQuery.data ? usageQuery.data.reduce((a, b) => a + b.session_count, 0) : "—"}
          icon={<TbBolt size={16} />}
          accent="amber"
        />
      </div>

      {/* шкала фокуса и разбивка по категориям */}
      <div className="grid grid-cols-3 gap-4">
        <Card className="flex flex-col items-center justify-center py-6">
          {scoreLoading && <p className="text-sm text-muted">{t.calculating}</p>}
          {!scoreLoading && <ScoreGauge score={focusScore} size={150} />}
        </Card>

        <Card className="col-span-2">
          <CardHeader title={t.timeBreakdown} />
          {stats ? (
            <div className="flex items-center gap-6">
              <DonutChart slices={donutSlices} size={120} thickness={22} />
              <div className="flex-1">
                <DonutLegend slices={donutSlices} total={stats.total} />
                <div className="mt-3 pt-3 border-t border-zinc-100 dark:border-zinc-800 grid grid-cols-3 gap-2 text-center">
                  {donutSlices.map((s) => (
                    <div key={s.label}>
                      <p className="text-xs font-bold" style={{ color: s.color }}>
                        {fmtMsShort(s.value)}
                      </p>
                      <p className="text-[10px] text-muted">{s.label}</p>
                    </div>
                  ))}
                </div>
              </div>
            </div>
          ) : (
            <p className="text-sm text-muted">
              {usageQuery.isLoading ? t.loading : t.noDataYet}
            </p>
          )}
        </Card>
      </div>

      {/* топ активности: приложения + сайты */}
      <Card>
        <CardHeader
          title={t.topActivityToday}
          subtitle={`${combinedActivity.length} ${t.tracked}`}
        />
        {(usageQuery.isLoading && browserQuery.isLoading) && (
          <p className="text-sm text-muted">{t.loading}</p>
        )}
        {combinedActivity.length === 0 && !usageQuery.isLoading && (
          <p className="text-sm text-muted">{t.noDataYet}</p>
        )}
        <div className="space-y-2">
          {combinedActivity.map((item) => {
            const color =
              item.category === "productive" ? "#22c55e"
              : item.category === "distraction" ? "#ef4444"
              : "#71717a";
            const pct = maxActivityMs > 0 ? (item.total_ms / maxActivityMs) * 100 : 0;
            return (
              <div key={item.key} className="flex items-center gap-2.5">
                <span className={[
                  "flex items-center gap-0.5 text-[9px] font-bold uppercase px-1.5 py-0.5 rounded shrink-0",
                  item.type === "site"
                    ? "bg-sky-100 text-sky-600 dark:bg-sky-900/30 dark:text-sky-400"
                    : "bg-zinc-100 text-zinc-500 dark:bg-zinc-800 dark:text-zinc-400",
                ].join(" ")}>
                  {item.type === "site" ? <TbWorld size={9} /> : <TbApps size={9} />}
                  {item.type === "site" ? "web" : "app"}
                </span>
                <span className="text-sm text-zinc-700 dark:text-zinc-300 w-28 truncate shrink-0">
                  {item.name}
                </span>
                <div className="flex-1 bg-zinc-100 dark:bg-zinc-800 rounded-full h-1.5 overflow-hidden">
                  <div
                    className="h-full rounded-full transition-all duration-700"
                    style={{ width: `${pct}%`, background: color }}
                  />
                </div>
                <span className="text-xs font-mono text-muted w-12 text-right shrink-0">
                  {fmtMs(item.total_ms)}
                </span>
              </div>
            );
          })}
        </div>
      </Card>
    </div>
  );
}
