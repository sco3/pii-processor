mod common;

pub use common::init_logging::init_tracing;
use ductaper::env_vars::Cfg;
use ductaper::redact_consumer::RedactConsumer;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

#[tokio::test]
async fn test_consumer() {
    init_tracing();
    unsafe {
        std::env::set_var("TENANT", "T");
        std::env::set_var("APPLICATION", "A");
    }
    let cfg = Cfg::from_env();
    let consumer = RedactConsumer::new(&cfg).await;
    sleep(Duration::from_secs(2)).await;
    info!("Client: {:?}", consumer.client);
    info!("Jetstream: {:?}", consumer.jetstream);
}
