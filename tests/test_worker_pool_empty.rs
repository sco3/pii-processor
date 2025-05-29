mod common;

use common::test_worker_pool_common::test_pool;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::test]
pub async fn test_empty_payload() {
    let payload = Vec::new();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;
    tp.consumer_stop.stop_tx.send(()).unwrap()
}
#[tokio::test]
pub async fn test_empty_payload1() {
    let payload = "{}".as_bytes().to_vec();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;
    tp.consumer_stop.stop_tx.send(()).unwrap()
}
#[tokio::test]
pub async fn test_empty_payload2() {
    let payload = "[]".as_bytes().to_vec();
    let tp = test_pool(payload).await;
    sleep(Duration::from_millis(42)).await;
    tp.consumer_stop.stop_tx.send(()).unwrap()
}
