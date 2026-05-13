import { useQuery } from "@tanstack/react-query";
import { api } from "@/ipc/commands";
import type { AppUsageStat, SessionInfo } from "@/types/activity";

export function useActivityLog(date: string) {
  return useQuery<SessionInfo[], Error>({
    queryKey: ["activity", date],
    queryFn: () => api.getActivityLog(date),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });
}

export function useAppUsage(date: string) {
  return useQuery<AppUsageStat[], Error>({
    queryKey: ["appUsage", date],
    queryFn: () => api.getAppUsage(date),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });
}

export function useFocusScore(date: string) {
  return useQuery<number, Error>({
    queryKey: ["focusScore", date],
    queryFn: () => api.getFocusScore(date),
    staleTime: 30_000,
    refetchOnWindowFocus: false,
  });
}
