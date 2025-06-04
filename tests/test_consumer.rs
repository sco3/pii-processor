mod common;

use crate::common::init_cfg::get_test_cfg;
pub use common::init_logging::init_tracing;
use ductaper::redact_consumer::RedactConsumer;
use reqwest::StatusCode;
use std::sync::atomic::Ordering;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use tokio;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::info;

#[tokio::test]
async fn test_consumer() {
    init_tracing();
    unsafe {
        std::env::remove_var("DOCKER_HOST");
    }

    let container = GenericImage::new("nats", "2.11.4")
        .with_exposed_port(4222.tcp())
        .with_wait_for(WaitFor::http(
            HttpWaitStrategy::new("/healthz")
                .with_port(8222.tcp()) //
                .with_expected_status_code(StatusCode::OK),
        ))
        .with_network("bridge")
        .start()
        .await
        .expect("Failed to start Nats");

    if let Ok(port) = container.get_host_port_ipv4(4222.tcp()).await {
        info!("Container port: {port}");

        let cfg = get_test_cfg(port);
        let mut consumer = RedactConsumer::new(&cfg).await;
        consumer.update_stream(&cfg).await;
        consumer.subscribe(&cfg).await;
        let run = consumer.get_run_flag_clone();
        let _ = tokio::join!(
            async {
                sleep(TokioDuration::from_secs(2)).await;
                run.store(false, Ordering::Relaxed);
            },
            consumer.serve(),
        );
    }
}
