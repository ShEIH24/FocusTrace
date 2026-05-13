import type { ReactNode } from "react";

interface StatCardProps {
  label: string;
  value: string | number;
  sub?: string;
  icon?: ReactNode;
  trend?: { value: number; label: string };
  accent?: "green" | "red" | "amber" | "indigo";
}

const accentColors = {
  green:  "text-green-500",
  red:    "text-red-500",
  amber:  "text-amber-500",
  indigo: "text-indigo-500",
};

export function StatCard({ label, value, sub, icon, trend, accent = "indigo" }: StatCardProps) {
  return (
    <div className="card p-4 flex flex-col gap-2 animate-fade-in">
      <div className="flex items-center justify-between">
        <p className="text-label">{label}</p>
        {icon && <span className={`${accentColors[accent]} opacity-70`}>{icon}</span>}
      </div>
      <p className={`text-2xl font-bold tracking-tight ${accentColors[accent]}`}>{value}</p>
      <div className="flex items-center gap-2">
        {sub && <p className="text-xs text-muted">{sub}</p>}
        {trend && (
          <span className={`text-xs font-medium ${trend.value >= 0 ? "text-green-500" : "text-red-500"}`}>
            {trend.value >= 0 ? "↑" : "↓"} {Math.abs(trend.value)}% {trend.label}
          </span>
        )}
      </div>
    </div>
  );
}
