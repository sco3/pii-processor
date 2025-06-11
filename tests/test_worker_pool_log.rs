mod common;
use crate::common::test_worker_pool_common::test_pool;
use bytes::Bytes;
use ductaper::llm_work::preview::preview;
use std::fs;
use std::sync::atomic::Ordering;
use std::time::Duration;
use tokio::time::sleep;
use tracing::debug;

async fn read_session_log_file() -> Vec<u8> {
    let path = "tests/data/worker-pool-test.json";
    let file_content = fs::read(path) //
        .expect("Failed to read example_new_fields.log");

    let preview: Bytes = preview(&file_content);
    debug!("Session log preview: {:?}", preview);
    file_content
}

#[tokio::test]

pub async fn test_with_log() {
    let payload = read_session_log_file().await;
    let tp = test_pool(payload).await;
    sleep(Duration::from_secs(42 / 10)).await;

    tp.run_flag.store(false, Ordering::Relaxed);
}
