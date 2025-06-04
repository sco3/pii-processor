mod common;
pub use common::init_logging::init_tracing;
use ductaper::env_vars::Cfg;
use ductaper::redact_consumer::RedactConsumer;
use ductaper::secret_string::SecretString;
use reqwest::StatusCode;
use testcontainers::{
    GenericImage, ImageExt,
    core::wait::HttpWaitStrategy,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

use tracing::info;

#[tokio::test]
async fn test_consumer() {
    init_tracing();

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
        let cfg = Cfg {
            llm_token: SecretString::new("sk-1234"),
            log_level: "debug".to_string(),
            redact_subject: "redact".to_string(),
            queue_stream: "queue".to_string(),
            queue_stream_max_age: 3600,
            nats_url: format!("nats://localhost:{port}"),
            tenant: "tenant".to_string(),
            application: "application".to_string(),
        };
        let consumer = RedactConsumer::new(&cfg).await;
        info!("Client: {:?}", consumer.client);
        info!("Jetstream: {:?}", consumer.jetstream);
    }
}
