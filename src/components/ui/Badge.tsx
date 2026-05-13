import { useT } from "@/hooks/useT";
import type { Category } from "@/types/activity";

const categoryStyles: Record<Category, string> = {
  Productive: "bg-green-500/10 text-green-600 dark:text-green-400 ring-1 ring-green-500/20",
  Neutral:    "bg-zinc-500/10 text-zinc-600 dark:text-zinc-400 ring-1 ring-zinc-500/20",
  Distraction:"bg-red-500/10 text-red-600 dark:text-red-400 ring-1 ring-red-500/20",
};

export function CategoryBadge({ category }: { category: Category }) {
  const t = useT();

  // переводим название категории для отображения
  const labelMap: Record<Category, string> = {
    Productive: t.categoryProductive,
    Neutral:    t.categoryNeutral,
    Distraction: t.categoryDistraction,
  };

  return (
    <span className={`inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium ${categoryStyles[category]}`}>
      {labelMap[category]}
    </span>
  );
}

interface BadgeProps {
  children: React.ReactNode;
  variant?: "default" | "success" | "warning" | "danger" | "accent";
}

const variantStyles = {
  default: "bg-zinc-100 dark:bg-zinc-800 text-zinc-600 dark:text-zinc-400",
  success: "bg-green-500/10 text-green-600 dark:text-green-400",
  warning: "bg-amber-500/10 text-amber-600 dark:text-amber-400",
  danger:  "bg-red-500/10 text-red-600 dark:text-red-400",
  accent:  "bg-accent-500/10 text-accent-600 dark:text-indigo-400",
};

export function Badge({ children, variant = "default" }: BadgeProps) {
  return (
    <span className={`inline-flex items-center px-2 py-0.5 rounded-md text-xs font-medium ${variantStyles[variant]}`}>
      {children}
    </span>
  );
}
