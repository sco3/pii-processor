mod common;

pub use common::init_logging::init_tracing;
use ductaper::s3helper::S3Helper;
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
    .with_env_var("initialBuckets", "test-bucket")
    .start()
    .await
    .expect("Failed to start s3");

    if let Ok(port) = container.get_host_port_ipv4(9090.tcp()).await {
        info!("Port {port}");
        if let Ok(s3) = S3Helper::new(
            "test".to_string(),
            "eu-west-1".to_string(), //
            None,
            None,
            None,
            Some(format!("http://localhost{port}")),
        )
        .await
        {
            s3.list_buckets().await;
        }
    }
}
