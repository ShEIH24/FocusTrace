import { useQuery } from "@tanstack/react-query";
import { api } from "@/ipc/commands";
import { useUiStore } from "@/store/uiStore";
import { useT } from "@/hooks/useT";
import { Card, CardHeader } from "@/components/ui/Card";
import { StatCard } from "@/components/ui/StatCard";
import { DonutChart, DonutLegend } from "@/components/charts/DonutChart";
import { HorizontalBar } from "@/components/charts/BarChart";
import { CategoryBadge } from "@/components/ui/Badge";
import {
  TbClock, TbFlame, TbTargetArrow, TbCalendar,
  TbChevronLeft, TbChevronRight, TbWorld,
} from "react-icons/tb";
import type { AppUsageStat, SessionInfo, Category } from "@/types/activity";
import type { BrowserEventRow, DomainStat } from "@/types/browser";

function fmtMs(ms: number): string {
  const h = Math.floor(ms / 3_600_000);
  const m = Math.floor((ms % 3_600_000) / 60_000);
  if (h > 0) return `${h}h ${m}m`;
  return `${m}m`;
}

function fmtTime(iso: string): string {
  return new Date(iso).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function stepDate(date: string, delta: number): string {
  const d = new Date(date);
  d.setDate(d.getDate() + delta);
  return d.toISOString().slice(0, 10);
}

function normalizeCat(cat: string): Category {
  const l = cat.toLowerCase();
  if (l === "productive") return "Productive";
  if (l === "distraction") return "Distraction";
  return "Neutral";
}

function categoryTotals(stats: AppUsageStat[]) {
  let productive = 0, neutral = 0, distraction = 0;
  for (const s of stats) {
    if (s.category === "Productive") productive += s.total_ms;
    else if (s.category === "Neutral") neutral += s.total_ms;
    else distraction += s.total_ms;
  }
  const total = productive + neutral + distraction;
  return { productive, neutral, distraction, total };
}

function peakHour(sessions: SessionInfo[]): string {
  if (!sessions.length) return "—";
  const hourMap = new Map<number, number>();
  for (const s of sessions) {
    const h = new Date(s.started_at).getHours();
    hourMap.set(h, (hourMap.get(h) ?? 0) + s.duration_ms);
  }
  let best = 0, bestMs = 0;
  hourMap.forEach((ms, h) => { if (ms > bestMs) { bestMs = ms; best = h; } });
  return `${String(best).padStart(2, "0")}:00`;
}

type LogEntry = {
  key: string;
  name: string;
  title: string;
  started_at: string;
  duration_ms: number;
  category: Category;
  type: "app" | "site";
};

export default function DailyReport() {
  const { selectedDate, setDate } = useUiStore();
  const t = useT();
  const todayStr = new Date().toISOString().slice(0, 10);

  const usageQuery = useQuery<AppUsageStat[], Error>({
    queryKey: ["appUsage", selectedDate],
    queryFn: () => api.getAppUsage(selectedDate),
    staleTime: 30_000,
  });

  const logQuery = useQuery<SessionInfo[], Error>({
    queryKey: ["activityLog", selectedDate],
    queryFn: () => api.getActivityLog(selectedDate),
    staleTime: 30_000,
  });

  const scoreQuery = useQuery<number, Error>({
    queryKey: ["focusScore", selectedDate],
    queryFn: () => api.getFocusScore(selectedDate),
    staleTime: 30_000,
  });

  const browserHistoryQuery = useQuery<BrowserEventRow[], Error>({
    queryKey: ["browserHistory", selectedDate],
    queryFn: () => api.getBrowserHistory(selectedDate),
    staleTime: 30_000,
  });

  const browserStatsQuery = useQuery<DomainStat[], Error>({
    queryKey: ["browserDomainStats", selectedDate],
    queryFn: () => api.getBrowserDomainStats(selectedDate),
    staleTime: 30_000,
  });

  const stats = usageQuery.data ? categoryTotals(usageQuery.data) : null;
  const sessions = logQuery.data ?? [];

  // объединённый топ активности: приложения + сайты
  type ActivityItem = { key: string; name: string; total_ms: number; category: string };
  const allActivity: ActivityItem[] = [
    ...(usageQuery.data ?? []).map((a) => ({
      key: `app:${a.exe}`,
      name: a.exe,
      total_ms: a.total_ms,
      category: a.category,
    })),
    ...(browserStatsQuery.data ?? []).map((d) => ({
      key: `site:${d.domain}`,
      name: d.domain,
      total_ms: d.durationMs,
      category: normalizeCat(d.category),
    })),
  ].sort((a, b) => b.total_ms - a.total_ms).slice(0, 8);

  const maxActivityMs = allActivity[0]?.total_ms ?? 1;

  const itemColor = (cat: string) => {
    const c = cat.toLowerCase();
    return c === "productive" ? "#22c55e" : c === "distraction" ? "#ef4444" : "#71717a";
  };

  // объединённый журнал: приложения + браузерные события, отсортированные по времени
  const logEntries: LogEntry[] = [
    ...(logQuery.data ?? []).map((s, i) => ({
      key: `app-${i}-${s.started_at}`,
      name: s.exe,
      title: s.title,
      started_at: s.started_at,
      duration_ms: s.duration_ms,
      category: s.category,
      type: "app" as const,
    })),
    ...(browserHistoryQuery.data ?? []).map((e) => ({
      key: `site-${e.id}`,
      name: e.domain,
      title: e.title,
      started_at: e.startedAt,
      duration_ms: e.durationMs,
      category: normalizeCat(e.category),
      type: "site" as const,
    })),
  ].sort((a, b) => new Date(a.started_at).getTime() - new Date(b.started_at).getTime());

  const donutSlices = stats
    ? [
        { label: t.categoryProductive,  value: stats.productive,  color: "#22c55e" },
        { label: t.categoryNeutral,      value: stats.neutral,     color: "#71717a" },
        { label: t.categoryDistraction,  value: stats.distraction, color: "#ef4444" },
      ]
    : [];

  const displayDate = new Date(selectedDate).toLocaleDateString(t.locale, {
    weekday: "long", month: "long", day: "numeric", year: "numeric",
  });

  return (
    <div className="p-6 space-y-5 max-w-4xl mx-auto animate-fade-in">
      {/* заголовок с переключением дат */}
      <div className="flex items-start justify-between">
        <div>
          <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">{t.dailyReportTitle}</h1>
          <p className="text-sm text-muted mt-0.5">{displayDate}</p>
        </div>
        <div className="flex items-center gap-2">
          <button onClick={() => setDate(stepDate(selectedDate, -1))} className="btn-ghost p-2">
            <TbChevronLeft size={16} />
          </button>
          <div className="relative flex items-center">
            <TbCalendar size={14} className="absolute left-3 text-zinc-400 pointer-events-none" />
            <input
              type="date"
              value={selectedDate}
              max={todayStr}
              onChange={(e) => setDate(e.target.value)}
              className="input-base pl-8 py-1.5 w-40"
            />
          </div>
          <button
            onClick={() => setDate(stepDate(selectedDate, 1))}
            disabled={selectedDate >= todayStr}
            className="btn-ghost p-2 disabled:opacity-30"
          >
            <TbChevronRight size={16} />
          </button>
        </div>
      </div>

      {/* статистика */}
      <div className="grid grid-cols-4 gap-3">
        <StatCard
          label={t.focusScore}
          value={scoreQuery.data !== undefined ? Math.round(scoreQuery.data) : "—"}
          sub={t.outOf100}
          icon={<TbTargetArrow size={16} />}
          accent="indigo"
        />
        <StatCard
          label={t.totalTime}
          value={stats ? fmtMs(stats.total) : "—"}
          icon={<TbClock size={16} />}
          accent="green"
        />
        <StatCard
          label={t.productive}
          value={stats?.total ? `${Math.round((stats.productive / stats.total) * 100)}%` : "—"}
          sub={stats ? fmtMs(stats.productive) : undefined}
          icon={<TbFlame size={16} />}
          accent="green"
        />
        <StatCard
          label={t.peakHour}
          value={peakHour(sessions)}
          sub={t.mostActive}
          icon={<TbClock size={16} />}
          accent="amber"
        />
      </div>

      {/* пончик и топ активности */}
      <div className="grid grid-cols-5 gap-4">
        <Card className="col-span-2">
          <CardHeader title={t.categorySplit} />
          {stats && stats.total > 0 ? (
            <div className="flex flex-col items-center gap-4">
              <div className="relative">
                <DonutChart slices={donutSlices} size={140} thickness={26} />
                <div className="absolute inset-0 flex items-center justify-center">
                  <div className="text-center mt-2">
                    <p className="text-lg font-bold text-zinc-900 dark:text-zinc-100">
                      {stats.total > 0 ? Math.round((stats.productive / stats.total) * 100) : 0}%
                    </p>
                    <p className="text-[9px] text-muted">{t.categoryProductive}</p>
                  </div>
                </div>
              </div>
              <DonutLegend slices={donutSlices} total={stats.total} />
            </div>
          ) : (
            <p className="text-sm text-muted">{usageQuery.isLoading ? t.loading : t.noData}</p>
          )}
        </Card>

        <Card className="col-span-3">
          <CardHeader
            title={t.topActivity}
            subtitle={`${allActivity.length} ${t.tracked}`}
          />
          <div className="space-y-2.5">
            {allActivity.length === 0 && (
              <p className="text-sm text-muted">{t.noAppData}</p>
            )}
            {allActivity.map((item) => (
              <HorizontalBar
                key={item.key}
                label={item.name}
                value={item.total_ms}
                max={maxActivityMs}
                color={itemColor(item.category)}
                suffix={fmtMs(item.total_ms)}
              />
            ))}
          </div>
        </Card>
      </div>

      {/* объединённый журнал: приложения + сайты */}
      <Card>
        <CardHeader
          title={t.sessionLog}
          subtitle={`${logEntries.length} ${t.sessionsWord}`}
        />
        {(logQuery.isLoading && browserHistoryQuery.isLoading) && (
          <p className="text-sm text-muted">{t.loading}</p>
        )}
        {logEntries.length === 0 && !logQuery.isLoading && (
          <p className="text-sm text-muted">{t.noSessionsRecorded}</p>
        )}
        <div className="space-y-0 divide-y divide-zinc-100 dark:divide-zinc-800/60 -mx-5">
          {logEntries.slice(0, 20).map((entry) => (
            <div
              key={entry.key}
              className="flex items-center gap-3 px-5 py-2.5 hover:bg-zinc-50 dark:hover:bg-zinc-800/40 transition-colors"
            >
              <div className="text-[10px] font-mono text-muted w-10 flex-shrink-0">
                {fmtTime(entry.started_at)}
              </div>
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-1.5">
                  {entry.type === "site" && (
                    <TbWorld size={11} className="text-sky-500 shrink-0" />
                  )}
                  <span className="text-sm font-medium text-zinc-800 dark:text-zinc-200 truncate">
                    {entry.name}
                  </span>
                </div>
                <span className="text-xs text-muted truncate block">{entry.title || "—"}</span>
              </div>
              <CategoryBadge category={entry.category} />
              <span className="text-xs font-mono text-muted w-12 text-right flex-shrink-0">
                {fmtMs(entry.duration_ms)}
              </span>
            </div>
          ))}
          {logEntries.length > 20 && (
            <p className="px-5 py-3 text-xs text-muted text-center">
              +{logEntries.length - 20} {t.moreSessions}
            </p>
          )}
        </div>
      </Card>
    </div>
  );
}
