mod common;

pub use common::init_logging::init_tracing;
use ductaper::s3ctx::S3Ctx;
use reqwest::StatusCode;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};

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
    .with_env_var("debug", "false")
    .start()
    .await
    .expect("Failed to start s3");

    if let Ok(port) = container.get_host_port_ipv4(9090.tcp()).await {
        info!("aws s3api list-buckets --endpoint-url=http://localhost:{port}");
        let test_bucket = "test-bucket".to_string();
        if let Ok(s3) = S3Ctx::new(
            test_bucket.clone(),
            "eu-west-1".to_string(), //
            None,
            None,
            None,
            Some(format!("http://localhost:{port}")),
        )
        .await
        {
            let ls = s3.list_buckets().await;
            assert_eq!(ls.len(), 1);
            if let Some(name) = ls.get(0) {
                assert_eq!(*name, test_bucket)
            } else {
                panic!("Wrong bucket was found!");
            }
        }
    }
}
