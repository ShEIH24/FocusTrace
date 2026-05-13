import { useEffect } from "react";
import { onIdleChanged } from "@/ipc/events";
import { useActivityStore } from "@/store/activityStore";

/**
 * Subscribes to `idle-changed` Tauri events and syncs them into the activity store.
 * Kept for backward compatibility — `useRealtimeUpdates` supersedes this when mounted
 * at the layout level, but callers that mount this directly still work correctly
 * (the event can have multiple listeners with no side effects).
 */
export function useIdleStatus(): void {
  const setIdleStatus = useActivityStore((s) => s.setIdleStatus);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    onIdleChanged((payload) => {
      if (payload.isIdle) {
        setIdleStatus(true, payload.at ? new Date(payload.at) : new Date());
      } else {
        setIdleStatus(false);
      }
    }).then((fn) => {
      unlisten = fn;
    });

    return () => {
      unlisten?.();
    };
  }, [setIdleStatus]);
}
