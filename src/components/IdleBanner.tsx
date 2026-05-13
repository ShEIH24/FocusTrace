import { useEffect, useState } from "react";
import { useActivityStore } from "@/store/activityStore";
import { useT } from "@/hooks/useT";
import { TbMoonStars } from "react-icons/tb";

export default function IdleBanner() {
  const isIdle = useActivityStore((s) => s.isIdle);
  const idleSince = useActivityStore((s) => s.idleSince);
  const [elapsed, setElapsed] = useState(0);
  const t = useT();

  useEffect(() => {
    if (!isIdle) { setElapsed(0); return; }
    const id = setInterval(() => {
      setElapsed(idleSince ? Math.floor((Date.now() - idleSince.getTime()) / 1000) : 0);
    }, 1000);
    return () => clearInterval(id);
  }, [isIdle, idleSince]);

  if (!isIdle) return null;

  const m = Math.floor(elapsed / 60);
  const s = elapsed % 60;
  const sinceLabel = idleSince
    ? idleSince.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" })
    : "—";

  return (
    <div className="flex items-center gap-3 px-4 py-3 rounded-xl
      bg-amber-50 dark:bg-amber-500/5 border border-amber-200 dark:border-amber-500/20
      animate-fade-in">
      <TbMoonStars size={18} className="text-amber-500 flex-shrink-0" />
      <div className="flex-1 min-w-0">
        <p className="text-sm font-semibold text-amber-800 dark:text-amber-300">{t.awayFromKeyboard}</p>
        <p className="text-xs text-amber-600 dark:text-amber-400/80">
          {t.idleSinceLabel} {sinceLabel} — {m}:{String(s).padStart(2, "0")} {t.idleLabel}
        </p>
      </div>
    </div>
  );
}
