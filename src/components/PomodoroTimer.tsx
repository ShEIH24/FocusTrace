import { usePomodoroStore } from "@/store/pomodoroStore";
import { useT } from "@/hooks/useT";
import { useUiStore } from "@/store/uiStore";
import { api } from "@/ipc/commands";
import { sessionCountLabel } from "@/i18n/translations";
import type { PomodoroPhase } from "@/types/productivity";

const PHASE_COLOR: Record<PomodoroPhase, string> = {
  idle: "stroke-zinc-600",
  working: "stroke-indigo-500",
  short_break: "stroke-emerald-500",
  long_break: "stroke-teal-500",
};

const PHASE_TEXT_COLOR: Record<PomodoroPhase, string> = {
  idle: "text-zinc-400",
  working: "text-indigo-400",
  short_break: "text-emerald-400",
  long_break: "text-teal-400",
};

function formatTime(secs: number): string {
  const m = Math.floor(secs / 60).toString().padStart(2, "0");
  const s = (secs % 60).toString().padStart(2, "0");
  return `${m}:${s}`;
}

// кольцо прогресса
const RADIUS = 88;
const CIRCUM = 2 * Math.PI * RADIUS;

function Ring({ phase, progress }: { phase: PomodoroPhase; progress: number }) {
  const offset = CIRCUM * (1 - progress);
  return (
    <svg className="w-52 h-52" viewBox="0 0 200 200">
      <circle cx="100" cy="100" r={RADIUS} fill="none" stroke="currentColor" strokeWidth="8"
        className="text-zinc-800" />
      <circle
        cx="100" cy="100" r={RADIUS}
        fill="none"
        strokeWidth="8"
        strokeLinecap="round"
        strokeDasharray={CIRCUM}
        strokeDashoffset={offset}
        transform="rotate(-90 100 100)"
        className={`${PHASE_COLOR[phase]} transition-[stroke-dashoffset] duration-700 ease-linear`}
      />
    </svg>
  );
}

// точки завершённых сессий
function SessionDots({ count, target = 4 }: { count: number; target?: number }) {
  return (
    <div className="flex gap-1.5 items-center">
      {Array.from({ length: target }).map((_, i) => (
        <span
          key={i}
          className={`w-2 h-2 rounded-full transition-colors duration-300 ${
            i < count % target ? "bg-indigo-500" : "bg-zinc-700"
          }`}
        />
      ))}
    </div>
  );
}

export default function PomodoroTimer() {
  const { phase, remainingSecs, totalSecs, sessionCount, isPaused } = usePomodoroStore();
  const language = useUiStore((s) => s.language);
  const t = useT();

  // метки фаз зависят от языка
  const phaseLabels: Record<PomodoroPhase, string> = {
    idle: t.pomodoroReady,
    working: t.pomodoroWorking,
    short_break: t.pomodoroShortBreak,
    long_break: t.pomodoroLongBreak,
  };

  const progress = totalSecs > 0 ? remainingSecs / totalSecs : 1;
  const isIdle = phase === "idle";
  const isRunning = !isIdle && !isPaused;

  const handlePrimary = () => {
    if (isIdle) return api.pomodoroStart();
    if (isPaused) return api.pomodoroResume();
    return api.pomodoroPause();
  };

  return (
    <div className="flex flex-col items-center gap-6">
      {/* кольцо с обратным отсчётом */}
      <div className="relative flex items-center justify-center">
        <Ring phase={phase} progress={progress} />
        <div className="absolute flex flex-col items-center gap-0.5">
          <span className="text-4xl font-mono font-semibold tabular-nums text-zinc-900 dark:text-zinc-100">
            {isIdle ? "--:--" : formatTime(remainingSecs)}
          </span>
          <span className={`text-xs font-medium uppercase tracking-widest ${PHASE_TEXT_COLOR[phase]}`}>
            {phaseLabels[phase]}
            {isPaused && !isIdle ? ` · ${t.pausedLabel}` : ""}
          </span>
        </div>
      </div>

      {/* точки сессий */}
      <SessionDots count={sessionCount} />

      {/* кнопки управления */}
      <div className="flex items-center gap-3">
        <button
          onClick={handlePrimary}
          className="px-6 py-2.5 rounded-xl font-medium text-sm bg-indigo-600 hover:bg-indigo-500 text-white transition-colors"
        >
          {isIdle ? t.start : isRunning ? t.pause : t.resume}
        </button>

        {!isIdle && (
          <>
            <button
              onClick={() => api.pomodoroSkip()}
              className="px-4 py-2.5 rounded-xl text-sm font-medium text-zinc-400 hover:text-zinc-200 bg-zinc-800 hover:bg-zinc-700 transition-colors"
            >
              {t.skip}
            </button>
            <button
              onClick={() => api.pomodoroStop()}
              className="px-4 py-2.5 rounded-xl text-sm font-medium text-zinc-400 hover:text-red-400 bg-zinc-800 hover:bg-zinc-700 transition-colors"
            >
              {t.stop}
            </button>
          </>
        )}
      </div>

      {/* счётчик завершённых сессий */}
      <p className="text-xs text-zinc-500">
        {sessionCountLabel(sessionCount, language)}
      </p>
    </div>
  );
}
