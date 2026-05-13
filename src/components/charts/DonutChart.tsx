interface Slice {
  label: string;
  value: number;
  color: string;
}

interface DonutChartProps {
  slices: Slice[];
  size?: number;
  thickness?: number;
}

export function DonutChart({ slices, size = 140, thickness = 24 }: DonutChartProps) {
  const r = (size - thickness) / 2;
  const cx = size / 2;
  const cy = size / 2;
  const circumference = 2 * Math.PI * r;
  const total = slices.reduce((s, d) => s + d.value, 0);

  if (total === 0) {
    return (
      <svg width={size} height={size}>
        <circle cx={cx} cy={cy} r={r} fill="none" stroke="#3f3f46" strokeWidth={thickness} />
      </svg>
    );
  }

  let offset = 0;
  const segments = slices.map((slice) => {
    const pct = slice.value / total;
    const dash = pct * circumference;
    const gap = circumference - dash;
    const seg = { ...slice, dash, gap, offset, pct };
    offset += dash;
    return seg;
  });

  return (
    <svg width={size} height={size} style={{ transform: "rotate(-90deg)" }}>
      <circle
        cx={cx} cy={cy} r={r}
        fill="none"
        stroke="currentColor"
        strokeWidth={thickness}
        className="text-zinc-100 dark:text-zinc-800"
      />
      {segments.map((seg) => (
        <circle
          key={seg.label}
          cx={cx} cy={cy} r={r}
          fill="none"
          stroke={seg.color}
          strokeWidth={thickness}
          strokeDasharray={`${seg.dash} ${seg.gap}`}
          strokeDashoffset={-seg.offset}
          strokeLinecap="butt"
          style={{ transition: "stroke-dasharray 0.5s ease" }}
        />
      ))}
    </svg>
  );
}

interface DonutLegendProps {
  slices: Slice[];
  total: number;
}

export function DonutLegend({ slices, total }: DonutLegendProps) {
  return (
    <div className="flex flex-col gap-2">
      {slices.map((slice) => (
        <div key={slice.label} className="flex items-center justify-between gap-3 text-sm">
          <div className="flex items-center gap-2">
            <span className="w-2.5 h-2.5 rounded-sm flex-shrink-0" style={{ background: slice.color }} />
            <span className="text-zinc-700 dark:text-zinc-300">{slice.label}</span>
          </div>
          <div className="text-right">
            <span className="font-semibold text-zinc-900 dark:text-zinc-100">
              {total > 0 ? Math.round((slice.value / total) * 100) : 0}%
            </span>
          </div>
        </div>
      ))}
    </div>
  );
}
