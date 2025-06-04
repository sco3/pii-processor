mod common;

pub use common::init_logging::init_tracing;
use ductaper::redact_consumer::RedactConsumer;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::test]
async fn test_consumer() {
    init_tracing();
    let consumer = RedactConsumer::new("nats://localhost:4222").await;
    sleep(Duration::from_secs(2)).await;
    info!("Client: {:?}", consumer.client);
}
