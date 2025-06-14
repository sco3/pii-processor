mod common;

use common::test_worker_pool_common::test_pool;
use std::sync::atomic::Ordering;
use tokio::time::{Duration, sleep};

#[tokio::test]
pub async fn test_empty_payload() {
    let payload = Vec::new();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;

    tp.run_flag.store(false, Ordering::Relaxed);
}
#[tokio::test]
pub async fn test_empty_payload1() {
    let payload = "{}".as_bytes().to_vec();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;

    tp.run_flag.store(false, Ordering::Relaxed);
}
#[tokio::test]
pub async fn test_empty_payload2() {
    let payload = "[]".as_bytes().to_vec();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;

    tp.run_flag.store(false, Ordering::Relaxed);
}
