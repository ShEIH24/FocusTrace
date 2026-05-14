use chrono::{DateTime, Utc};

use crate::analytics::classifier::RulesClassifier;
use crate::events::types::{SessionInfo, WindowInfo};

/// Tracks the currently active window and produces `SessionInfo` records when
/// focus changes or the session is finalized.
pub struct SessionManager {
    classifier: RulesClassifier,
    current_window: Option<WindowInfo>,
    session_start: Option<DateTime<Utc>>,
}

impl SessionManager {
    pub fn new(classifier: RulesClassifier) -> Self {
        Self {
            classifier,
            current_window: None,
            session_start: None,
        }
    }

    /// Called when the active window changes. Returns the completed session if one existed.
    pub fn on_window_changed(&mut self, new_window: WindowInfo) -> Option<SessionInfo> {
        let now = Utc::now();
        let completed = self.flush(now);
        self.current_window = Some(new_window);
        self.session_start = Some(now);
        completed
    }

    /// Finalizes the current in-progress session.
    pub fn finalize(&mut self) -> Option<SessionInfo> {
        self.flush(Utc::now())
    }

    fn flush(&mut self, now: DateTime<Utc>) -> Option<SessionInfo> {
        let window = self.current_window.take()?;
        let start = self.session_start.take()?;
        let duration_ms = (now - start).num_milliseconds();
        if duration_ms < 200 {
            return None;
        }
        let category = self
            .classifier
            .classify(&window.exe, &window.exe_path)
            .to_string();
        Some(SessionInfo {
            exe: window.exe,
            title: window.title,
            exe_path: window.exe_path,
            started_at: start,
            duration_ms,
            category,
        })
    }
}
