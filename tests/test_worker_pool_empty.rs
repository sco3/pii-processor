mod common;

use common::test_worker_pool_common::test_pool;
use std::sync::atomic::Ordering;
use tokio::time::{Duration, sleep};
use tracing::info;

#[tokio::test]
pub async fn test_empty_payload() {
    let tp = test_pool(Vec::new()).await;
    sleep(Duration::from_millis(42)).await;

    tp.run_flag.store(false, Ordering::Relaxed);
    let count = tp.pool.counter.count_last_hour();
    info!("Count: {}", count);
}
