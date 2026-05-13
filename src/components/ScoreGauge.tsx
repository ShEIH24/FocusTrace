import { useT } from "@/hooks/useT";
import type { Translations } from "@/i18n/translations";

interface ScoreGaugeProps {
  score: number;
  size?: number;
}

function scoreColor(score: number): string {
  if (score < 40) return "#ef4444";
  if (score < 70) return "#f59e0b";
  return "#22c55e";
}

// возвращает метку уровня фокуса из переводов
function scoreLabel(score: number, t: Translations): string {
  if (score < 40) return t.gaugeDistracted;
  if (score < 70) return t.gaugeModerate;
  if (score < 90) return t.gaugeFocused;
  return t.gaugeDeepFocus;
}

export default function ScoreGauge({ score, size = 160 }: ScoreGaugeProps) {
  const t = useT();
  const r = (size / 2) * 0.74;
  const cx = size / 2;
  const cy = size / 2;
  const strokeWidth = size * 0.11;
  const circumference = Math.PI * r;
  const clamped = Math.max(0, Math.min(100, score));
  const filled = (clamped / 100) * circumference;

  const startX = cx - r;
  const startY = cy;
  const endX = cx + r;
  const arc = `M ${startX} ${startY} A ${r} ${r} 0 0 1 ${endX} ${startY}`;

  return (
    <div className="flex flex-col items-center gap-1">
      <svg width={size} height={size / 2 + strokeWidth} overflow="visible">
        <path d={arc} fill="none" stroke="currentColor" strokeWidth={strokeWidth} strokeLinecap="round"
          className="text-zinc-100 dark:text-zinc-800" />
        <path
          d={arc} fill="none"
          stroke={scoreColor(clamped)}
          strokeWidth={strokeWidth}
          strokeLinecap="round"
          strokeDasharray={`${filled} ${circumference - filled}`}
          style={{ transition: "stroke-dasharray 0.7s cubic-bezier(.4,0,.2,1), stroke 0.4s ease" }}
        />
        <text x={cx} y={cy + 2} textAnchor="middle" dominantBaseline="middle"
          fontSize={size * 0.2} fontWeight="700" fill={scoreColor(clamped)}>
          {Math.round(clamped)}
        </text>
      </svg>
      <p className="text-xs font-semibold" style={{ color: scoreColor(clamped) }}>{scoreLabel(clamped, t)}</p>
      <p className="text-[10px] text-muted">{t.gaugeFocusScore}</p>
    </div>
  );
}
