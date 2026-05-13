interface HeatmapCell {
  day: string;   // e.g. "Mon"
  hour: number;  // 0-23
  value: number; // 0-100
}

interface HeatmapProps {
  cells: HeatmapCell[];
  days: string[];
}

function intensityColor(v: number): string {
  if (v === 0) return "transparent";
  if (v < 25) return "rgba(99,102,241,0.15)";
  if (v < 50) return "rgba(99,102,241,0.35)";
  if (v < 75) return "rgba(99,102,241,0.6)";
  return "rgba(99,102,241,0.9)";
}

const WORK_HOURS = [8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21];

export function ActivityHeatmap({ cells, days }: HeatmapProps) {
  const cellMap = new Map<string, number>();
  cells.forEach((c) => cellMap.set(`${c.day}-${c.hour}`, c.value));

  const cellSize = 20;
  const cellGap = 3;
  const labelW = 28;
  const labelH = 20;

  const totalW = labelW + days.length * (cellSize + cellGap);
  const totalH = labelH + WORK_HOURS.length * (cellSize + cellGap);

  return (
    <div className="overflow-x-auto">
      <svg width={totalW} height={totalH}>
        {days.map((day, di) => (
          <text
            key={day}
            x={labelW + di * (cellSize + cellGap) + cellSize / 2}
            y={12}
            textAnchor="middle"
            fontSize={9}
            fill="#71717a"
          >
            {day}
          </text>
        ))}
        {WORK_HOURS.map((hour, hi) => (
          <text
            key={hour}
            x={labelW - 4}
            y={labelH + hi * (cellSize + cellGap) + cellSize / 2 + 4}
            textAnchor="end"
            fontSize={9}
            fill="#71717a"
          >
            {String(hour).padStart(2, "0")}
          </text>
        ))}
        {WORK_HOURS.map((hour, hi) =>
          days.map((day, di) => {
            const v = cellMap.get(`${day}-${hour}`) ?? 0;
            return (
              <rect
                key={`${day}-${hour}`}
                x={labelW + di * (cellSize + cellGap)}
                y={labelH + hi * (cellSize + cellGap)}
                width={cellSize}
                height={cellSize}
                rx={3}
                fill={v > 0 ? intensityColor(v) : "transparent"}
                className="stroke-zinc-200 dark:stroke-zinc-800"
                strokeWidth={1}
              >
                <title>{`${day} ${String(hour).padStart(2, "0")}:00 — ${v}%`}</title>
              </rect>
            );
          })
        )}
      </svg>
    </div>
  );
}

interface WeekHeatmapProps {
  data: { date: string; score: number }[];
  locale?: string;
}

export function WeekHeatmap({ data, locale = "en-US" }: WeekHeatmapProps) {
  const cellSize = 28;

  function scoreToColor(score: number): string {
    if (score === 0) return "transparent";
    if (score < 40) return "rgba(239,68,68,0.25)";
    if (score < 70) return "rgba(245,158,11,0.35)";
    return "rgba(34,197,94,0.45)";
  }

  return (
    <div className="flex gap-1.5 items-end">
      {data.map((d) => {
        const dayLabel = new Date(d.date).toLocaleDateString(locale, { weekday: "short" }).slice(0, 2);
        return (
          <div key={d.date} className="flex flex-col items-center gap-1">
            <div
              className="rounded-md border border-zinc-200 dark:border-zinc-700 transition-all"
              style={{
                width: cellSize,
                height: cellSize,
                background: scoreToColor(d.score),
              }}
              title={`${d.date}: ${Math.round(d.score)}`}
            />
            <span className="text-[9px] text-zinc-400">{dayLabel}</span>
          </div>
        );
      })}
    </div>
  );
}
