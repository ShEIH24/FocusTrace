import { useQuery } from "@tanstack/react-query";
import { api } from "@/ipc/commands";
import { useT } from "@/hooks/useT";
import type { GoalProgress as GoalProgressType } from "@/types/productivity";

function Bar({ pct, color }: { pct: number; color: string }) {
  const clamped = Math.min(100, Math.round(pct));
  return (
    <div className="relative h-1.5 w-full rounded-full bg-zinc-800">
      <div
        className={`absolute inset-y-0 left-0 rounded-full transition-all duration-500 ${color}`}
        style={{ width: `${clamped}%` }}
      />
    </div>
  );
}

function GoalRow({
  label, actual, target, pct, color,
}: {
  label: string; actual: string; target: string; pct: number; color: string;
}) {
  const met = pct >= 100;
  return (
    <div className="flex flex-col gap-1.5">
      <div className="flex items-center justify-between text-xs">
        <span className="text-zinc-400">{label}</span>
        <span className={`font-medium tabular-nums ${met ? "text-emerald-400" : "text-zinc-300"}`}>
          {actual}
          <span className="text-zinc-600 font-normal"> / {target}</span>
        </span>
      </div>
      <Bar pct={pct} color={met ? "bg-emerald-500" : color} />
    </div>
  );
}

function msToHm(ms: number): string {
  const totalMin = Math.floor(ms / 60000);
  const h = Math.floor(totalMin / 60);
  const m = totalMin % 60;
  return h > 0 ? `${h}h ${m}m` : `${m}m`;
}

export default function GoalProgress() {
  const t = useT();
  const { data } = useQuery<GoalProgressType>({
    queryKey: ["dailyGoals"],
    queryFn: () => api.getDailyGoals(),
    refetchInterval: 60_000,
  });

  if (!data) {
    return (
      <div className="flex flex-col gap-3">
        {[0, 1, 2].map((i) => (
          <div key={i} className="h-8 rounded-lg bg-zinc-800 animate-pulse" />
        ))}
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-4">
      <GoalRow
        label={t.productiveTime}
        actual={msToHm(data.productiveActualMs)}
        target={msToHm(data.productiveTargetMs)}
        pct={data.productivePct}
        color="bg-indigo-500"
      />
      <GoalRow
        label={t.pomodoros}
        actual={String(data.pomodoroActual)}
        target={String(data.pomodoroTarget)}
        pct={data.pomodoroPct}
        color="bg-violet-500"
      />
      <GoalRow
        label={t.focusScore}
        actual={`${Math.round(data.scoreActual)}`}
        target={`${Math.round(data.scoreTarget)}`}
        pct={data.scorePct}
        color="bg-sky-500"
      />

      {data.overallMet && (
        <p className="text-xs text-emerald-400 font-medium pt-0.5">{t.allGoalsMet}</p>
      )}
    </div>
  );
}
