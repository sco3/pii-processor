use reqwest::StatusCode;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::core::{IntoContainerPort, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt};
#[allow(dead_code)]
/// starts nats container for tests
pub async fn get_nats_container() -> ContainerAsync<GenericImage> {
    GenericImage::new(
        "nats", "2.11.4", //
    )
    .with_exposed_port(4222.tcp())
    .with_wait_for(WaitFor::http(
        HttpWaitStrategy::new("/healthz")
            .with_port(8222.tcp()) //
            .with_expected_status_code(StatusCode::OK),
    ))
    .with_network("bridge")
    .with_cmd(["-js", "-m", "8222"])
    .start()
    .await
    .expect("Failed to start Nats")
}
