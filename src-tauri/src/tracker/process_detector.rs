use windows::Win32::Foundation::{CloseHandle, FALSE};
use windows::Win32::System::Threading::{
    OpenProcess, QueryFullProcessImageNameW, PROCESS_NAME_WIN32, PROCESS_QUERY_LIMITED_INFORMATION,
};

#[derive(Debug, Clone)]
pub struct ProcessInfo {
    pub pid: u32,
    /// Executable filename only (e.g. `"Code.exe"`).
    pub name: String,
    /// Full Win32 path (e.g. `"C:\...\Code.exe"`).
    pub exe_path: String,
}

/// Returns process metadata for `pid` using `QueryFullProcessImageNameW`.
///
/// Returns `None` if the process cannot be opened or its path cannot be read
/// (e.g. protected system processes).
pub fn get_process_info(pid: u32) -> Option<ProcessInfo> {
    unsafe {
        let handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, FALSE, pid).ok()?;

        let mut buf = vec![0u16; 1024];
        let mut size = buf.len() as u32;

        let ok = QueryFullProcessImageNameW(
            handle,
            PROCESS_NAME_WIN32,
            windows::core::PWSTR(buf.as_mut_ptr()),
            &mut size,
        );
        let _ = CloseHandle(handle);

        if ok.is_err() || size == 0 {
            return None;
        }

        let exe_path = String::from_utf16_lossy(&buf[..size as usize]).to_string();
        let name = exe_path
            .rsplit(['/', '\\'])
            .next()
            .unwrap_or("unknown")
            .to_string();

        Some(ProcessInfo {
            pid,
            name,
            exe_path,
        })
    }
}
