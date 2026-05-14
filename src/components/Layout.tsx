import { useEffect } from "react";
import { Outlet } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import Sidebar from "./Sidebar";
import UpdateBanner from "./UpdateBanner";
import { useUiStore } from "@/store/uiStore";
import { useActivityStore } from "@/store/activityStore";
import { usePomodoroStore } from "@/store/pomodoroStore";
import { useRealtimeUpdates } from "@/hooks/useRealtimeUpdates";
import { useTauriEvent } from "@/hooks/useTauriEvents";
import { api } from "@/ipc/commands";
import type { WindowInfo } from "@/types/activity";

const CURRENT_VERSION = "0.1.0";
const GITHUB_REPO = "ShEIH24/FocusTrace";

function isNewerVersion(latest: string, current: string): boolean {
  const parse = (v: string) => v.split(".").map(Number);
  const [lMaj = 0, lMin = 0, lPatch = 0] = parse(latest);
  const [cMaj = 0, cMin = 0, cPatch = 0] = parse(current);
  if (lMaj !== cMaj) return lMaj > cMaj;
  if (lMin !== cMin) return lMin > cMin;
  return lPatch > cPatch;
}

export default function Layout() {
  const theme = useUiStore((s) => s.theme);
  const setPendingUpdateVersion = useUiStore((s) => s.setPendingUpdateVersion);
  const pendingUpdateVersion = useUiStore((s) => s.pendingUpdateVersion);
  const setCurrentWindow = useActivityStore((s) => s.setCurrentWindow);
  const applyTick = usePomodoroStore((s) => s.applyTick);

  useRealtimeUpdates();
  useTauriEvent<string>("update-available", setPendingUpdateVersion);

  // Проверка обновлений через GitHub Releases API через 10 с после старта.
  useEffect(() => {
    const controller = new AbortController();
    const timer = setTimeout(async () => {
      try {
        const res = await fetch(
          `https://api.github.com/repos/${GITHUB_REPO}/releases/latest`,
          { signal: controller.signal, headers: { Accept: "application/vnd.github+json" } },
        );
        if (!res.ok) return;
        const data: { tag_name?: string } = await res.json();
        const latest = (data.tag_name ?? "").replace(/^v/, "");
        if (latest && isNewerVersion(latest, CURRENT_VERSION)) {
          setPendingUpdateVersion(latest);
        }
      } catch {
        // ignore network errors
      }
    }, 10_000);
    return () => { clearTimeout(timer); controller.abort(); };
  }, [setPendingUpdateVersion]);

  // Poll the pomodoro state every second from the root layout so the
  // Sidebar timer and the Focus page stay in sync on every page.
  const { data: pomodoroState } = useQuery({
    queryKey: ["pomodoroState"],
    queryFn: () => api.getPomodoroState(),
    refetchInterval: 1000,
    staleTime: 0,
    gcTime: 0,
  });

  useEffect(() => {
    if (pomodoroState) applyTick(pomodoroState);
  }, [pomodoroState, applyTick]);

  // Populate the active window immediately from the backend cache so
  // the ActivityCard is never empty on startup.
  useEffect(() => {
    api.getLiveCache().then((snapshot) => {
      if (snapshot.currentWindow) {
        const w: WindowInfo = {
          exe: snapshot.currentWindow.exe,
          title: snapshot.currentWindow.title,
          pid: snapshot.currentWindow.pid,
        };
        setCurrentWindow(w);
      }
    }).catch(() => {/* ignore startup errors */});
  // Run once on mount only.
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const root = document.documentElement;
    if (theme === "dark") {
      root.classList.add("dark");
    } else if (theme === "light") {
      root.classList.remove("dark");
    } else {
      const prefersDark = window.matchMedia("(prefers-color-scheme: dark)").matches;
      root.classList.toggle("dark", prefersDark);
    }
  }, [theme]);

  return (
    <div className="flex h-screen bg-zinc-50 dark:bg-zinc-950 text-zinc-900 dark:text-zinc-100 overflow-hidden">
      <Sidebar />
      <main className="flex-1 overflow-y-auto">
        {pendingUpdateVersion && (
          <div className="px-6 pt-4">
            <UpdateBanner />
          </div>
        )}
        <Outlet />
      </main>
    </div>
  );
}
