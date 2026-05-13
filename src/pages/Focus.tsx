import { useState } from "react";
import { useQuery, useMutation, useQueryClient } from "@tanstack/react-query";
import { TbTarget, TbFlame, TbPlayerPlay, TbBrain } from "react-icons/tb";

import { api } from "@/ipc/commands";
import { useT } from "@/hooks/useT";
import PomodoroTimer from "@/components/PomodoroTimer";
import GoalProgress from "@/components/GoalProgress";
import StreakCard from "@/components/StreakCard";
import type { ProductivityConfig, PomodoroConfig } from "@/types/productivity";

// ── переключатель режима фокуса ───────────────────────────────────────────

function FocusModeToggle() {
  const qc = useQueryClient();

  const { data: enabled = false } = useQuery({
    queryKey: ["focusModeState"],
    queryFn: () => api.getFocusModeState(),
  });

  const toggle = useMutation({
    mutationFn: () => api.toggleFocusMode(),
    onSuccess: (next) => {
      qc.setQueryData(["focusModeState"], next);
    },
  });

  return (
    <button
      onClick={() => toggle.mutate()}
      className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors duration-200 ${
        enabled ? "bg-indigo-600" : "bg-zinc-700"
      }`}
    >
      <span
        className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform duration-200 ${
          enabled ? "translate-x-6" : "translate-x-1"
        }`}
      />
    </button>
  );
}

// ── числовое поле настройки ───────────────────────────────────────────────

interface NumFieldProps {
  label: string;
  value: number;
  onChange: (v: number) => void;
  min?: number;
  max?: number;
}

function NumField({ label, value, onChange, min = 1, max = 120 }: NumFieldProps) {
  return (
    <div className="flex flex-col gap-1">
      <label className="text-xs text-zinc-500">{label}</label>
      <input
        type="number"
        min={min}
        max={max}
        value={value}
        onChange={(e) => onChange(Number(e.target.value))}
        className="input-base w-full"
      />
    </div>
  );
}

// ── панель настроек таймера ───────────────────────────────────────────────

function ConfigPanel({
  config,
  onSave,
}: {
  config: ProductivityConfig;
  onSave: (c: ProductivityConfig) => void;
}) {
  const [local, setLocal] = useState(config);
  const t = useT();

  // удобные хелперы для обновления вложенных полей
  function setPomo<K extends keyof PomodoroConfig>(key: K, value: PomodoroConfig[K]) {
    setLocal((prev) => ({ ...prev, pomodoro: { ...prev.pomodoro, [key]: value } }));
  }

  function setGoalTarget(value: number) {
    setLocal((prev) => ({ ...prev, goals: { ...prev.goals, pomodoroTarget: value } }));
  }

  return (
    <div className="flex flex-col gap-5">
      <div className="grid grid-cols-2 gap-4">
        <NumField
          label={t.work}
          value={local.pomodoro.workMins}
          onChange={(v) => setPomo("workMins", v)}
        />
        <NumField
          label={t.shortBreak}
          value={local.pomodoro.shortBreakMins}
          onChange={(v) => setPomo("shortBreakMins", v)}
        />
        <NumField
          label={t.longBreak}
          value={local.pomodoro.longBreakMins}
          onChange={(v) => setPomo("longBreakMins", v)}
        />
        <NumField
          label={t.sessionsUntilLong}
          value={local.pomodoro.sessionsUntilLong}
          min={1}
          max={16}
          onChange={(v) => setPomo("sessionsUntilLong", v)}
        />
      </div>

      <NumField
        label={t.dailyPomodoroTarget}
        value={local.goals.pomodoroTarget}
        min={0}
        max={32}
        onChange={setGoalTarget}
      />

      <div className="flex items-center justify-between">
        <span className="text-sm text-zinc-300">{t.autoStartBreaks}</span>
        <input
          type="checkbox"
          checked={local.pomodoro.autoStartBreaks}
          onChange={(e) => setPomo("autoStartBreaks", e.target.checked)}
          className="w-4 h-4 accent-indigo-500"
        />
      </div>

      <div className="flex items-center justify-between">
        <span className="text-sm text-zinc-300">{t.autoStartWork}</span>
        <input
          type="checkbox"
          checked={local.pomodoro.autoStartWork}
          onChange={(e) => setPomo("autoStartWork", e.target.checked)}
          className="w-4 h-4 accent-indigo-500"
        />
      </div>

      <button onClick={() => onSave(local)} className="btn-primary self-end">
        {t.save}
      </button>
    </div>
  );
}

// ── страница ──────────────────────────────────────────────────────────────

export default function Focus() {
  const qc = useQueryClient();
  const t = useT();

  const { data: config } = useQuery<ProductivityConfig>({
    queryKey: ["productivityConfig"],
    queryFn: () => api.getProductivityConfig(),
  });

  const saveConfig = useMutation({
    mutationFn: (cfg: ProductivityConfig) => api.updateProductivityConfig(cfg),
    onSuccess: (_, cfg) => {
      qc.setQueryData(["productivityConfig"], cfg);
    },
  });

  return (
    <div className="flex flex-col gap-6 p-6 max-w-2xl mx-auto">
      {/* заголовок */}
      <div>
        <h1 className="text-xl font-semibold text-zinc-100">{t.focusTitle}</h1>
        <p className="text-sm text-zinc-500 mt-0.5">{t.pomodoroAndGoals}</p>
      </div>

      {/* таймер и цели */}
      <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
        {/* карточка таймера */}
        <div className="card flex flex-col items-center py-8">
          <PomodoroTimer />
        </div>

        {/* цели и серия */}
        <div className="flex flex-col gap-4">
          <div className="card flex flex-col gap-4">
            <div className="flex items-center gap-2">
              <TbTarget className="w-4 h-4 text-indigo-400" />
              <span className="text-sm font-medium text-zinc-300">{t.todaysGoals}</span>
            </div>
            <GoalProgress />
          </div>

          <div className="card flex flex-col gap-4">
            <div className="flex items-center gap-2">
              <TbFlame className="w-4 h-4 text-orange-400" />
              <span className="text-sm font-medium text-zinc-300">{t.streak}</span>
            </div>
            <StreakCard />
          </div>
        </div>
      </div>

      {/* режим фокуса */}
      <div className="card flex items-center justify-between">
        <div className="flex items-center gap-3">
          <TbBrain className="w-5 h-5 text-violet-400" />
          <div>
            <p className="text-sm font-medium text-zinc-200">{t.focusMode}</p>
            <p className="text-xs text-zinc-500">{t.focusModeDesc}</p>
          </div>
        </div>
        <FocusModeToggle />
      </div>

      {/* настройки таймера */}
      {config && (
        <div className="card flex flex-col gap-4">
          <div className="flex items-center gap-2">
            <TbPlayerPlay className="w-4 h-4 text-zinc-400" />
            <span className="text-sm font-medium text-zinc-300">{t.timerSettings}</span>
          </div>
          <ConfigPanel config={config} onSave={(cfg) => saveConfig.mutate(cfg)} />
        </div>
      )}
    </div>
  );
}
