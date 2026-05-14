use std::time::Instant;
use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tracing::debug;

use crate::events::types::{AppEvent, WindowInfo};
use crate::tracker::win32::get_foreground_window_info;

/// A new window must be continuously focused for this long before we emit
/// `WindowChanged`. Filters out rapid alt-tab noise.
const DEBOUNCE_MS: u64 = 150;

/// Poll interval while a candidate is pending or shortly after a switch.
const FAST_POLL_MS: u64 = 200;

/// How long to stay in fast-poll mode after the last confirmed switch.
/// Covers the common pattern of quickly switching between two apps.
const FAST_COOLDOWN_MS: u128 = 2_000;

/// Two windows are "the same" when they share a PID and identical title.
/// PID is more precise than exe name (handles multiple instances), while
/// title catches document / tab changes within the same process.
fn same_window(a: &Option<WindowInfo>, b: &Option<WindowInfo>) -> bool {
    match (a, b) {
        (Some(a), Some(b)) => a.pid == b.pid && a.title == b.title,
        (None, None) => true,
        _ => false,
    }
}

/// Polls the foreground window and emits `WindowChanged` only after the new
/// window has remained focused for at least `DEBOUNCE_MS` milliseconds.
///
/// Adaptive sleep:
/// - `FAST_POLL_MS`   while a candidate is debouncing or within `FAST_COOLDOWN_MS`
///   of the last confirmed switch.
/// - `poll_interval_ms` (from config, default 1 s) during stable periods.
pub async fn run(tx: mpsc::Sender<AppEvent>, poll_interval_ms: u64) {
    // Last window for which we emitted WindowChanged.
    let mut confirmed: Option<WindowInfo> = None;
    // Candidate window + the instant it first appeared (debounce clock).
    let mut candidate: Option<(WindowInfo, Instant)> = None;
    // When we last emitted a confirmed switch (drives fast-poll cooldown).
    let mut last_confirmed_at: Option<Instant> = None;

    loop {
        let current = get_foreground_window_info();

        if same_window(&confirmed, &current) {
            // Foreground is unchanged — discard any stale candidate.
            candidate = None;
        } else {
            // Foreground differs from confirmed; decide whether to debounce.
            let same_as_candidate = candidate.as_ref().is_some_and(|(cand, _)| match &current {
                Some(cur) => cand.pid == cur.pid && cand.title == cur.title,
                None => false,
            });

            if same_as_candidate {
                // Candidate is holding steady — check if debounce period elapsed.
                if let Some((_, since)) = &candidate {
                    if since.elapsed() >= Duration::from_millis(DEBOUNCE_MS) {
                        if let Some(info) = current.clone() {
                            debug!(
                                exe = %info.exe,
                                title = %info.title,
                                path = %info.exe_path,
                                "window confirmed"
                            );
                            if tx
                                .send(AppEvent::WindowChanged(info.clone()))
                                .await
                                .is_err()
                            {
                                break;
                            }
                            confirmed = Some(info);
                            candidate = None;
                            last_confirmed_at = Some(Instant::now());
                        }
                    }
                    // else: still within debounce window — nothing to do this tick.
                }
            } else {
                // Window changed before debounce elapsed (or there was no candidate yet).
                // Start / restart the debounce clock for the new window.
                candidate = current.map(|w| (w, Instant::now()));
            }
        }

        // Use the fast interval while debouncing or in the post-switch cooldown zone
        // so we catch rapid multi-step switches with low latency. Fall back to the
        // configured (slower) interval during idle stable periods to save CPU.
        let in_fast_zone =
            last_confirmed_at.is_some_and(|t| t.elapsed().as_millis() < FAST_COOLDOWN_MS);

        let sleep_ms = if candidate.is_some() || in_fast_zone {
            FAST_POLL_MS
        } else {
            poll_interval_ms
        };

        sleep(Duration::from_millis(sleep_ms)).await;
    }
}
