interface Point { label: string; value: number }

interface LineChartProps {
  points: Point[];
  height?: number;
  color?: string;
  areaFill?: boolean;
}

function smooth(pts: [number, number][]): string {
  if (pts.length < 2) return pts.map(([x, y]) => `${x},${y}`).join(" ");
  let d = `M ${pts[0][0]},${pts[0][1]}`;
  for (let i = 1; i < pts.length; i++) {
    const prev = pts[i - 1];
    const curr = pts[i];
    const cpx = (prev[0] + curr[0]) / 2;
    d += ` C ${cpx},${prev[1]} ${cpx},${curr[1]} ${curr[0]},${curr[1]}`;
  }
  return d;
}

export function LineChart({ points, height = 100, color = "#6366f1", areaFill = true }: LineChartProps) {
  if (points.length === 0) return null;
  const W = 100;
  const H = height;
  const pad = { t: 12, b: 6, l: 4, r: 4 };
  const max = Math.max(...points.map((p) => p.value), 1);
  const innerW = W - pad.l - pad.r;
  const innerH = H - pad.t - pad.b;

  const coords: [number, number][] = points.map((p, i) => [
    pad.l + (i / (points.length - 1)) * innerW,
    pad.t + (1 - p.value / max) * innerH,
  ]);

  const linePath = smooth(coords);
  const areaPath = `${linePath} L ${coords[coords.length - 1][0]},${pad.t + innerH} L ${coords[0][0]},${pad.t + innerH} Z`;

  const viewBox = `0 0 ${W} ${H}`;

  return (
    <div className="w-full overflow-hidden">
      <svg viewBox={viewBox} preserveAspectRatio="none" className="w-full" style={{ height }}>
        {areaFill && (
          <path d={areaPath} fill={color} fillOpacity={0.08} />
        )}
        <path d={linePath} fill="none" stroke={color} strokeWidth={1.5} strokeLinecap="round" strokeLinejoin="round" />
        {coords.map(([x, y], i) => (
          <circle key={i} cx={x} cy={y} r={2} fill={color} />
        ))}
      </svg>
      <div className="flex justify-between px-1 -mt-1">
        {points.map((p) => (
          <span key={p.label} className="text-[9px] text-zinc-500 leading-none shrink-0">
            {p.label}
          </span>
        ))}
      </div>
    </div>
  );
}
