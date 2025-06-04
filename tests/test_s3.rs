mod common;

pub use common::init_logging::init_tracing;
use reqwest::StatusCode;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use tokio;
use tracing::info;

#[tokio::test]
async fn test_s3() {
    init_tracing();
    unsafe {
        std::env::remove_var("DOCKER_HOST");
    }

    let container = GenericImage::new(
        "adobe/s3mock",
        "latest", //
    )
    .with_exposed_port(9090.tcp())
    .with_wait_for(WaitFor::http(
        HttpWaitStrategy::new("/")
            .with_port(9090.tcp()) //
            .with_expected_status_code(StatusCode::OK),
    ))
    .with_network("bridge")
    .start()
    .await
    .expect("Failed to start Nats");

    if let Ok(port) = container.get_host_port_ipv4(9090.tcp()).await {
        info!("Port {port}");
    }
}
