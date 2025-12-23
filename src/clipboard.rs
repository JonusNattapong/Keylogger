use clipboard::{ClipboardContext, ClipboardProvider};
use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::logger::Logger;
use crate::config::clipboard_interval;

pub async fn clipboard_monitor(logger: Arc<Logger>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    let mut last_content = String::new();

    loop {
        sleep(Duration::from_secs(clipboard_interval())).await;
        if let Ok(content) = ctx.get_contents() {
            if content != last_content {
                logger.log_clipboard(content.clone());
                last_content = content;
            }
        }
    }
}