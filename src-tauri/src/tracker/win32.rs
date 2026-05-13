use std::time::Duration;
use windows::Win32::System::SystemInformation::GetTickCount;
use windows::Win32::UI::Input::KeyboardAndMouse::{GetLastInputInfo, LASTINPUTINFO};
use windows::Win32::UI::WindowsAndMessaging::{
    GetForegroundWindow, GetWindowTextW, GetWindowThreadProcessId,
};

use crate::events::types::WindowInfo;
use crate::tracker::process_detector::get_process_info;

/// Returns info about the currently active foreground window, or `None` if
/// the window cannot be queried (e.g. no foreground window, protected process).
pub fn get_foreground_window_info() -> Option<WindowInfo> {
    unsafe {
        let hwnd = GetForegroundWindow();
        if hwnd.0 == 0 {
            return None;
        }

        let mut title_buf = vec![0u16; 512];
        let title_len = GetWindowTextW(hwnd, &mut title_buf);
        let title = String::from_utf16_lossy(&title_buf[..title_len.max(0) as usize]);

        let mut pid = 0u32;
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
        if pid == 0 {
            return None;
        }

        let proc = get_process_info(pid)?;

        Some(WindowInfo {
            exe: proc.name,
            title,
            pid: proc.pid,
            exe_path: proc.exe_path,
        })
    }
}

/// Returns the duration since the last keyboard or mouse input event.
pub fn get_idle_duration() -> Duration {
    unsafe {
        let mut info = LASTINPUTINFO {
            cbSize: std::mem::size_of::<LASTINPUTINFO>() as u32,
            dwTime: 0,
        };
        if GetLastInputInfo(&mut info).as_bool() {
            let tick = GetTickCount();
            let idle_ms = tick.wrapping_sub(info.dwTime);
            Duration::from_millis(idle_ms as u64)
        } else {
            Duration::ZERO
        }
    }
}
