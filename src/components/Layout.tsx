import { useEffect } from "react";
import { Outlet } from "react-router-dom";
import { useQuery } from "@tanstack/react-query";
import Sidebar from "./Sidebar";
import { useUiStore } from "@/store/uiStore";
import { useActivityStore } from "@/store/activityStore";
import { usePomodoroStore } from "@/store/pomodoroStore";
import { useRealtimeUpdates } from "@/hooks/useRealtimeUpdates";
import { useTauriEvent } from "@/hooks/useTauriEvents";
import { api } from "@/ipc/commands";
import type { WindowInfo } from "@/types/activity";

export default function Layout() {
  const theme = useUiStore((s) => s.theme);
  const setPendingUpdateVersion = useUiStore((s) => s.setPendingUpdateVersion);
  const setCurrentWindow = useActivityStore((s) => s.setCurrentWindow);
  const applyTick = usePomodoroStore((s) => s.applyTick);

  useRealtimeUpdates();
  useTauriEvent<string>("update-available", setPendingUpdateVersion);

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
        <Outlet />
      </main>
    </div>
  );
}
