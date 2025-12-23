use windows::Win32::UI::WindowsAndMessaging::{GetForegroundWindow, GetWindowTextW};
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;

pub fn get_active_window_title() -> String {
    unsafe {
        let hwnd = GetForegroundWindow();
        let mut buffer = [0u16; 512];
        let len = GetWindowTextW(hwnd, &mut buffer);
        OsString::from_wide(&buffer[..len as usize]).to_string_lossy().into_owned()
    }
}