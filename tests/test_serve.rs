use axum::http::StatusCode;
use ductaper::util::logging::init_tracing;
use std::sync::Arc;
use testcontainers::core::wait::HttpWaitStrategy;

use tracing::info;

mod common;

use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};

use crate::common::dummy_caller::DummyCaller;
use crate::common::dummy_saver::DummySaver;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::worker_pool::WorkerPool;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::debug;

#[tokio::test]
async fn test_serve() {
    init_tracing();

    let container = GenericImage::new("nats", "2.11.4")
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
        .expect("Failed to start Nats");

    if let Ok(port) = container.get_host_port_ipv4(4222.tcp()).await {
        info!("Container port: {port}");
    }

    let proc = LlmLogProcessor::new(
        Arc::new(DummyCaller {}),
        "Help if you can.".to_string(),
        "nova".to_string(),
        Arc::new(DummySaver::new()),
    );

    let (_tx, rx) = async_channel::bounded(1);

    let _wp = WorkerPool {
        size: 1,
        receiver: rx,
        llm_log_processor: Arc::new(proc),
        handlers: Vec::new(),
    };
    debug!("ok");
    sleep(TokioDuration::from_millis(1)).await;
}
