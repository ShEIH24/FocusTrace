use std::sync::Arc;
use std::time::Instant;

use chrono::Utc;
use tokio::sync::{mpsc, RwLock};
use tokio::time::{sleep, Duration};
use tracing::{debug, info};

use crate::config::app_config::AppConfig;
use crate::events::types::AppEvent;
use crate::tracker::win32::get_idle_duration;

// ---------------------------------------------------------------------------
// Polling strategy constants
// ---------------------------------------------------------------------------

/// Below this fraction of the threshold the detector uses the coarser base
/// interval — the user is clearly active, no need for tight polling.
const APPROACH_FACTOR: f64 = 0.75;

/// Tight poll used once idle time reaches `APPROACH_FACTOR * threshold`.
/// Limits threshold-crossing latency to at most this many seconds.
const FAST_POLL_SECS: u64 = 2;

/// Poll interval while the user is confirmed idle.
/// Short enough to feel responsive on resume; no need to go faster.
const IDLE_POLL_SECS: u64 = 5;

/// The base (active, far-from-threshold) interval is `threshold / POLL_DIVISOR`,
/// clamped to [`FAST_POLL_SECS`..`MAX_BASE_POLL_SECS`].
const POLL_DIVISOR: u64 = 6;

/// Hard cap on the base interval regardless of how large the threshold is.
const MAX_BASE_POLL_SECS: u64 = 10;

// ---------------------------------------------------------------------------
// Internal state machine
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Phase {
    Active,
    Idle,
}

// ---------------------------------------------------------------------------
// IdleDetector
// ---------------------------------------------------------------------------

/// Monitors combined keyboard + mouse idle time via `LASTINPUTINFO` and emits
/// [`AppEvent::IdleStarted`] / [`AppEvent::IdleEnded`] on state transitions.
///
/// **Thread safety** — `IdleDetector` is `Send` and owned by a single Tokio
/// task.  The `AppConfig` is shared via `Arc<RwLock<_>>` so the threshold can
/// be updated at runtime (e.g. from the settings page) without restarting.
///
/// **Corrected timestamps** — `IdleStarted.at` is back-dated by `idle_secs`
/// so it represents the true moment the user stopped providing input rather
/// than the moment the detector noticed the threshold had been crossed.
///
/// **Adaptive polling**
///
/// | State                        | Interval                          |
/// |------------------------------|-----------------------------------|
/// | Active, idle < 75 % of thr   | `clamp(thr / 6,  2 s .. 10 s)`   |
/// | Active, idle ≥ 75 % of thr   | 2 s  (precise crossing detection) |
/// | Idle                         | 5 s  (quick resume detection)     |
pub struct IdleDetector {
    config: Arc<RwLock<AppConfig>>,
    tx: mpsc::Sender<AppEvent>,
}

impl IdleDetector {
    pub fn new(config: Arc<RwLock<AppConfig>>, tx: mpsc::Sender<AppEvent>) -> Self {
        Self { config, tx }
    }

    /// Runs the detection loop until the event channel is closed (i.e. the
    /// coordinator has stopped consuming events).
    pub async fn run(self) {
        let mut phase = Phase::Active;

        // Monotonic timestamp taken at the moment we enter the Idle phase.
        let mut idle_onset: Option<Instant> = None;

        // idle_secs reading at onset — used to:
        //   1. back-date `IdleStarted.at` to the true user-stop moment, and
        //   2. compute the total idle duration on resume.
        let mut idle_secs_at_onset: u64 = 0;

        loop {
            // Re-read config every cycle so threshold changes apply immediately.
            let threshold_secs = self.config.read().await.idle_threshold_secs;
            let idle_secs = get_idle_duration().as_secs();

            match phase {
                Phase::Active => {
                    if idle_secs >= threshold_secs {
                        phase = Phase::Idle;
                        idle_onset = Some(Instant::now());
                        idle_secs_at_onset = idle_secs;

                        // Back-date: idle actually began `idle_secs` ago.
                        let at = Utc::now() - chrono::Duration::seconds(idle_secs as i64);

                        info!(
                            idle_secs,
                            threshold_secs,
                            started_at = %at.to_rfc3339(),
                            "idle started"
                        );

                        if self.tx.send(AppEvent::IdleStarted { at }).await.is_err() {
                            break;
                        }
                    }
                }

                Phase::Idle => {
                    if idle_secs < threshold_secs {
                        phase = Phase::Active;

                        // Total idle duration:
                        //   idle_secs_at_onset  — time idle before we detected the threshold
                        //   + elapsed_since_onset — time we spent in Idle phase
                        //   - idle_secs           — subtract the overlap: the user resumed
                        //                           somewhere in the last poll window, so
                        //                           `idle_secs` is the tail already counted
                        //                           inside `elapsed_since_onset`.
                        let elapsed = idle_onset.take().map_or(0, |t| t.elapsed().as_secs());
                        let duration_secs = idle_secs_at_onset + elapsed.saturating_sub(idle_secs);

                        info!(duration_secs, idle_secs, "idle ended");

                        if self
                            .tx
                            .send(AppEvent::IdleEnded { duration_secs })
                            .await
                            .is_err()
                        {
                            break;
                        }
                    }
                }
            }

            let sleep_secs = poll_interval(phase, idle_secs, threshold_secs);
            debug!(
                ?phase,
                idle_secs, threshold_secs, sleep_secs, "idle detector tick"
            );
            sleep(Duration::from_secs(sleep_secs)).await;
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Returns the next sleep duration in seconds.
///
/// Keeping this as a pure function makes the scheduling policy easy to unit-test
/// without running the async loop.
fn poll_interval(phase: Phase, idle_secs: u64, threshold_secs: u64) -> u64 {
    match phase {
        Phase::Idle => IDLE_POLL_SECS,

        Phase::Active => {
            let approach_at = (threshold_secs as f64 * APPROACH_FACTOR) as u64;
            if idle_secs >= approach_at {
                // Close to the threshold — poll tightly for accurate detection.
                FAST_POLL_SECS
            } else {
                // Well below the threshold — use a coarser interval to save CPU.
                (threshold_secs / POLL_DIVISOR).clamp(FAST_POLL_SECS, MAX_BASE_POLL_SECS)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn poll_interval_well_below_threshold() {
        // threshold=120 → base = 120/6 = 20, clamped to 10
        assert_eq!(poll_interval(Phase::Active, 0, 120), 10);
        assert_eq!(poll_interval(Phase::Active, 60, 120), 10);
    }

    #[test]
    fn poll_interval_approaching_threshold() {
        // 75 % of 120 = 90
        assert_eq!(poll_interval(Phase::Active, 90, 120), FAST_POLL_SECS);
        assert_eq!(poll_interval(Phase::Active, 119, 120), FAST_POLL_SECS);
    }

    #[test]
    fn poll_interval_idle_phase() {
        assert_eq!(poll_interval(Phase::Idle, 0, 120), IDLE_POLL_SECS);
        assert_eq!(poll_interval(Phase::Idle, 999, 120), IDLE_POLL_SECS);
    }

    #[test]
    fn poll_interval_small_threshold() {
        // threshold=10 → base = 10/6 = 1, clamped up to FAST_POLL_SECS(2)
        assert_eq!(poll_interval(Phase::Active, 0, 10), FAST_POLL_SECS);
        // approach_at = 7
        assert_eq!(poll_interval(Phase::Active, 8, 10), FAST_POLL_SECS);
    }

    #[test]
    fn poll_interval_large_threshold() {
        // threshold=600 → base = 600/6 = 100, clamped to MAX_BASE_POLL_SECS(10)
        assert_eq!(poll_interval(Phase::Active, 0, 600), MAX_BASE_POLL_SECS);
        // approach_at = 450
        assert_eq!(poll_interval(Phase::Active, 500, 600), FAST_POLL_SECS);
    }

    #[test]
    fn duration_formula_accuracy() {
        // idle_secs_at_onset=120, elapsed=12s, current_idle=2s → total=130s
        let idle_secs_at_onset: u64 = 120;
        let elapsed: u64 = 12;
        let current_idle: u64 = 2;
        let duration = idle_secs_at_onset + elapsed.saturating_sub(current_idle);
        assert_eq!(duration, 130);
    }

    #[test]
    fn duration_formula_no_underflow() {
        // elapsed smaller than current_idle: saturating_sub prevents underflow
        let duration = 120u64 + 1u64.saturating_sub(5u64);
        assert_eq!(duration, 120);
    }
}
