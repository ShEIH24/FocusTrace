import { NavLink } from "react-router-dom";
import {
  TbLayoutDashboard,
  TbTimeline,
  TbChartBar,
  TbSettings,
  TbReportAnalytics,
  TbBrain,
  TbTarget,
  TbSun,
  TbMoon,
} from "react-icons/tb";
import { useUiStore } from "@/store/uiStore";
import { usePomodoroStore } from "@/store/pomodoroStore";
import { useT } from "@/hooks/useT";
import type { PomodoroPhase } from "@/types/productivity";

const PHASE_DOT: Record<PomodoroPhase, string> = {
  idle: "bg-zinc-600",
  working: "bg-indigo-500 animate-pulse",
  short_break: "bg-emerald-500 animate-pulse",
  long_break: "bg-teal-500 animate-pulse",
};

function formatTime(secs: number): string {
  const m = Math.floor(secs / 60).toString().padStart(2, "0");
  const s = (secs % 60).toString().padStart(2, "0");
  return `${m}:${s}`;
}

export default function Sidebar() {
  const { theme, setTheme, language, setLanguage, pendingUpdateVersion } = useUiStore();
  const { phase, remainingSecs, isPaused } = usePomodoroStore();
  const t = useT();
  const timerActive = phase !== "idle";

  // пункты навигации собираем внутри компонента, чтобы использовать переводы
  const navItems = [
    { to: "/dashboard",    icon: <TbLayoutDashboard size={18} />, label: t.navDashboard },
    { to: "/timeline",     icon: <TbTimeline size={18} />,        label: t.navTimeline },
    { to: "/analytics",    icon: <TbChartBar size={18} />,        label: t.navAnalytics },
    { to: "/daily-report", icon: <TbReportAnalytics size={18} />, label: t.navDailyReport },
    { to: "/focus",        icon: <TbTarget size={18} />,          label: t.navFocus },
  ];

  function toggleTheme() {
    setTheme(theme === "dark" ? "light" : "dark");
  }

  return (
    <aside className="w-52 flex-shrink-0 flex flex-col border-r border-zinc-200 dark:border-zinc-800 bg-white dark:bg-zinc-950">
      {/* логотип */}
      <div className="flex items-center gap-2.5 px-4 py-4 border-b border-zinc-100 dark:border-zinc-800/60">
        <div className="w-7 h-7 rounded-lg bg-gradient-to-br from-indigo-500 to-violet-600 flex items-center justify-center flex-shrink-0">
          <TbBrain size={15} className="text-white" />
        </div>
        <div>
          <p className="text-sm font-bold text-zinc-900 dark:text-zinc-100 leading-none">FocusTrace</p>
          <p className="text-[10px] text-zinc-400 leading-none mt-0.5">{t.appSubtitle}</p>
        </div>
      </div>

      {/* навигация */}
      <nav className="flex-1 px-2 py-3 space-y-0.5">
        <p className="px-2 mb-2 text-[10px] font-semibold uppercase tracking-widest text-zinc-400 dark:text-zinc-600">
          {t.views}
        </p>
        {navItems.map((item) => (
          <NavLink
            key={item.to}
            to={item.to}
            className={({ isActive }) =>
              [
                "flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-sm font-medium transition-all duration-150",
                isActive
                  ? "bg-indigo-50 dark:bg-indigo-500/10 text-indigo-700 dark:text-indigo-400"
                  : "text-zinc-600 dark:text-zinc-400 hover:bg-zinc-50 dark:hover:bg-zinc-800/60 hover:text-zinc-900 dark:hover:text-zinc-200",
              ].join(" ")
            }
          >
            {({ isActive }) => (
              <>
                <span className={isActive ? "text-indigo-500" : ""}>{item.icon}</span>
                {item.label}
              </>
            )}
          </NavLink>
        ))}

        <div className="pt-3 mt-3 border-t border-zinc-100 dark:border-zinc-800/60">
          <p className="px-2 mb-2 text-[10px] font-semibold uppercase tracking-widest text-zinc-400 dark:text-zinc-600">
            {t.system}
          </p>
          <NavLink
            to="/settings"
            className={({ isActive }) =>
              [
                "flex items-center gap-2.5 px-2.5 py-2 rounded-lg text-sm font-medium transition-all duration-150",
                isActive
                  ? "bg-indigo-50 dark:bg-indigo-500/10 text-indigo-700 dark:text-indigo-400"
                  : "text-zinc-600 dark:text-zinc-400 hover:bg-zinc-50 dark:hover:bg-zinc-800/60 hover:text-zinc-900 dark:hover:text-zinc-200",
              ].join(" ")
            }
          >
            {({ isActive }) => (
              <>
                <span className="relative">
                  <span className={isActive ? "text-indigo-500" : ""}>
                    <TbSettings size={18} />
                  </span>
                  {pendingUpdateVersion && (
                    <span className="absolute -top-0.5 -right-0.5 w-2 h-2 rounded-full bg-amber-400 ring-2 ring-white dark:ring-zinc-950" />
                  )}
                </span>
                {t.navSettings}
              </>
            )}
          </NavLink>
        </div>
      </nav>

      {/* подвал с переключателями темы и языка */}
      <div className="px-3 py-3 border-t border-zinc-100 dark:border-zinc-800/60 flex flex-col gap-2">
        {/* компактный индикатор помодоро — виден только когда таймер запущен */}
        {timerActive && (
          <NavLink
            to="/focus"
            className="flex items-center gap-2 px-2.5 py-2 rounded-lg bg-indigo-500/10 text-indigo-400 hover:bg-indigo-500/20 transition-colors duration-150"
          >
            <span className={`w-2 h-2 rounded-full flex-shrink-0 ${PHASE_DOT[phase]}`} />
            <span className="text-xs font-mono font-medium tabular-nums flex-1">
              {formatTime(remainingSecs)}
            </span>
            {isPaused && <span className="text-[10px] text-zinc-500">{t.pausedLabel}</span>}
          </NavLink>
        )}

        {/* переключатель темы */}
        <button
          onClick={toggleTheme}
          className="w-full flex items-center gap-2 px-2.5 py-2 rounded-lg text-sm text-zinc-500 dark:text-zinc-400 hover:bg-zinc-50 dark:hover:bg-zinc-800 transition-all duration-150"
        >
          {theme === "dark" ? <TbSun size={16} /> : <TbMoon size={16} />}
          <span>{theme === "dark" ? t.lightMode : t.darkMode}</span>
        </button>

        {/* переключатель языка EN / RU */}
        <div className="flex gap-1 px-1">
          {(["en", "ru"] as const).map((lang) => (
            <button
              key={lang}
              onClick={() => setLanguage(lang)}
              className={[
                "flex-1 py-1.5 rounded-md text-xs font-semibold uppercase tracking-wide transition-all",
                language === lang
                  ? "bg-indigo-500/15 text-indigo-500 dark:text-indigo-400"
                  : "text-zinc-400 hover:text-zinc-600 dark:hover:text-zinc-300",
              ].join(" ")}
            >
              {lang}
            </button>
          ))}
        </div>
      </div>
    </aside>
  );
}
