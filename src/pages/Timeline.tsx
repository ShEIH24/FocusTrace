import { useQuery } from "@tanstack/react-query";
import { useUiStore } from "@/store/uiStore";
import { useActivityLog } from "@/hooks/useActivity";
import { useT } from "@/hooks/useT";
import { CategoryBadge } from "@/components/ui/Badge";
import { Card } from "@/components/ui/Card";
import { api } from "@/ipc/commands";
import { TbCalendar, TbChevronLeft, TbChevronRight, TbWorld } from "react-icons/tb";
import type { SessionInfo, Category } from "@/types/activity";
import type { BrowserEventRow } from "@/types/browser";

function fmtMs(ms: number): string {
  const s = Math.floor(ms / 1000);
  if (s < 60) return `${s}s`;
  const m = Math.floor(s / 60);
  if (m < 60) return `${m}m`;
  return `${Math.floor(m / 60)}h ${m % 60}m`;
}

function fmtTime(iso: string): string {
  return new Date(iso).toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
}

function normalizeCat(cat: string): Category {
  const l = cat.toLowerCase();
  if (l === "productive") return "Productive";
  if (l === "distraction") return "Distraction";
  return "Neutral";
}

type TimelineEntry = {
  key: string;
  name: string;
  title: string;
  started_at: string;
  duration_ms: number;
  category: Category;
  type: "app" | "site";
};

function buildEntries(sessions: SessionInfo[], browser: BrowserEventRow[]): TimelineEntry[] {
  const appEntries: TimelineEntry[] = sessions.map((s, i) => ({
    key: `app-${i}-${s.started_at}`,
    name: s.exe,
    title: s.title,
    started_at: s.started_at,
    duration_ms: s.duration_ms,
    category: s.category,
    type: "app",
  }));
  const siteEntries: TimelineEntry[] = browser.map((e) => ({
    key: `site-${e.id}`,
    name: e.domain,
    title: e.title,
    started_at: e.startedAt,
    duration_ms: e.durationMs,
    category: normalizeCat(e.category),
    type: "site",
  }));
  return [...appEntries, ...siteEntries].sort(
    (a, b) => new Date(b.started_at).getTime() - new Date(a.started_at).getTime(),
  );
}

function groupByHour(entries: TimelineEntry[]): Map<number, TimelineEntry[]> {
  const map = new Map<number, TimelineEntry[]>();
  for (const e of entries) {
    const hour = new Date(e.started_at).getHours();
    const g = map.get(hour) ?? [];
    g.push(e);
    map.set(hour, g);
  }
  return map;
}

const categoryBar: Record<string, string> = {
  Productive:  "bg-green-500",
  Neutral:     "bg-zinc-400 dark:bg-zinc-600",
  Distraction: "bg-red-500",
};

function TimelineRow({ entry }: { entry: TimelineEntry }) {
  return (
    <div className="flex items-center gap-3 py-2.5 px-3 rounded-lg hover:bg-zinc-50 dark:hover:bg-zinc-800/50 transition-colors group">
      <div className={`w-1 h-8 rounded-full flex-shrink-0 ${categoryBar[entry.category] ?? "bg-zinc-400"}`} />
      <div className="flex-1 min-w-0">
        <div className="flex items-center gap-1.5">
          {entry.type === "site" && (
            <TbWorld size={12} className="text-sky-500 shrink-0" />
          )}
          <p className="text-sm font-semibold text-zinc-900 dark:text-zinc-100 truncate">{entry.name}</p>
          <CategoryBadge category={entry.category} />
        </div>
        <p className="text-xs text-muted truncate mt-0.5">{entry.title || "—"}</p>
      </div>
      <div className="text-right flex-shrink-0 opacity-80 group-hover:opacity-100 transition-opacity">
        <p className="text-xs text-muted">{fmtTime(entry.started_at)}</p>
        <p className="text-xs font-mono font-semibold text-zinc-700 dark:text-zinc-300">{fmtMs(entry.duration_ms)}</p>
      </div>
    </div>
  );
}

function stepDate(date: string, delta: number): string {
  const d = new Date(date);
  d.setDate(d.getDate() + delta);
  return d.toISOString().slice(0, 10);
}

export default function Timeline() {
  const { selectedDate, setDate } = useUiStore();
  const { data: sessions, isLoading: sessionsLoading, isError } = useActivityLog(selectedDate);
  const t = useT();

  const browserQuery = useQuery<BrowserEventRow[], Error>({
    queryKey: ["browserHistory", selectedDate],
    queryFn: () => api.getBrowserHistory(selectedDate),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });

  const entries = buildEntries(sessions ?? [], browserQuery.data ?? []);
  const grouped = groupByHour(entries);
  const hours = Array.from(grouped.keys()).sort((a, b) => b - a);

  const todayStr = new Date().toISOString().slice(0, 10);
  const isToday = selectedDate === todayStr;
  const totalMs = entries.reduce((a, e) => a + e.duration_ms, 0);
  const isLoading = sessionsLoading && browserQuery.isLoading;

  return (
    <div className="p-6 space-y-5 max-w-3xl mx-auto animate-fade-in">
      <div className="flex items-center justify-between">
        <div>
          <h1 className="text-xl font-bold text-zinc-900 dark:text-zinc-100">{t.timelineTitle}</h1>
          <p className="text-sm text-muted mt-0.5">
            {entries.length} {t.sessionsWord} · {Math.floor(totalMs / 60_000)}m {t.tracked}
          </p>
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
              className="input-base pl-8 pr-3 py-1.5 w-40"
            />
          </div>
          <button
            onClick={() => setDate(stepDate(selectedDate, 1))}
            disabled={isToday}
            className="btn-ghost p-2 disabled:opacity-30"
          >
            <TbChevronRight size={16} />
          </button>
        </div>
      </div>

      {isLoading && (
        <div className="flex items-center justify-center py-16">
          <p className="text-sm text-muted">{t.loadingSessions}</p>
        </div>
      )}
      {isError && <p className="text-sm text-red-500">{t.failedToLoadSessions}</p>}

      {!isLoading && hours.length === 0 && (
        <Card>
          <div className="py-12 text-center">
            <p className="text-muted text-sm">{t.noSessionsForDay}</p>
          </div>
        </Card>
      )}

      <div className="space-y-4">
        {hours.map((hour) => (
          <div key={hour} className="animate-fade-in">
            <div className="flex items-center gap-3 mb-1.5">
              <p className="text-label">{String(hour).padStart(2, "0")}:00</p>
              <div className="flex-1 border-t border-zinc-200 dark:border-zinc-800" />
              <p className="text-[10px] text-muted">
                {grouped.get(hour)?.length} {t.sessionsWord}
              </p>
            </div>
            <Card padding="none" className="divide-y divide-zinc-100 dark:divide-zinc-800/60">
              {(grouped.get(hour) ?? []).map((entry) => (
                <TimelineRow key={entry.key} entry={entry} />
              ))}
            </Card>
          </div>
        ))}
      </div>
    </div>
  );
}
