interface Bar {
  label: string;
  value: number;
  color?: string;
}

interface BarChartProps {
  bars: Bar[];
  height?: number;
  showValues?: boolean;
}

function scoreColor(v: number, max: number): string {
  const pct = max > 0 ? v / max : 0;
  if (pct < 0.4) return "#ef4444";
  if (pct < 0.7) return "#f59e0b";
  return "#22c55e";
}

export function BarChart({ bars, height = 120, showValues = true }: BarChartProps) {
  const max = Math.max(...bars.map((b) => b.value), 1);
  const barW = 32;
  const gap = 10;
  const totalW = bars.length * (barW + gap) - gap;

  return (
    <div className="overflow-x-auto">
      <svg width={totalW} height={height + 36} className="overflow-visible">
        {bars.map((bar, i) => {
          const barH = Math.max((bar.value / max) * height, bar.value > 0 ? 3 : 0);
          const x = i * (barW + gap);
          const y = height - barH;
          const color = bar.color ?? scoreColor(bar.value, max);
          return (
            <g key={bar.label}>
              <rect
                x={x} y={height}
                width={barW} height={0}
                fill={color} rx={3} opacity={0.9}
                style={{ animation: `growUp 0.4s ease-out ${i * 0.05}s forwards` }}
              >
                <animate attributeName="height" from="0" to={barH} dur="0.4s" begin={`${i * 0.05}s`} fill="freeze" />
                <animate attributeName="y" from={height} to={y} dur="0.4s" begin={`${i * 0.05}s`} fill="freeze" />
              </rect>
              {showValues && bar.value > 0 && (
                <text x={x + barW / 2} y={y - 4} textAnchor="middle" fontSize={9} fontWeight="600" fill={color}>
                  {bar.value}
                </text>
              )}
              <text x={x + barW / 2} y={height + 16} textAnchor="middle" fontSize={10} className="fill-zinc-400">
                {bar.label}
              </text>
            </g>
          );
        })}
      </svg>
    </div>
  );
}

interface HorizontalBarProps {
  label: string;
  value: number;
  max: number;
  color?: string;
  suffix?: string;
}

export function HorizontalBar({ label, value, max, color = "#6366f1", suffix = "" }: HorizontalBarProps) {
  const pct = max > 0 ? (value / max) * 100 : 0;
  return (
    <div className="flex items-center gap-3">
      <span className="text-sm text-zinc-700 dark:text-zinc-300 w-32 truncate flex-shrink-0">{label}</span>
      <div className="flex-1 bg-zinc-100 dark:bg-zinc-800 rounded-full h-2 overflow-hidden">
        <div
          className="h-full rounded-full transition-all duration-700"
          style={{ width: `${pct}%`, background: color }}
        />
      </div>
      <span className="text-xs font-mono text-muted w-14 text-right flex-shrink-0">{suffix}</span>
    </div>
  );
}
