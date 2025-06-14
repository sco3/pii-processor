use axum::http::StatusCode;
use ductaper::util::logging::init_tracing;
use testcontainers::core::wait::HttpWaitStrategy;

use tracing::info;

mod common;

use crate::common::init_cfg::get_test_cfg;
use async_channel::{bounded, Receiver, Sender};
use async_nats::jetstream::Message;
use async_trait::async_trait;

use ductaper::llm_work::log_handler::LogHandler;
use ductaper::mq::connector::Connector;
use ductaper::mq::publisher::Publisher;
use ductaper::mq::redact_consumer::RedactConsumer;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};

use ductaper::mq::admin::StreamAdmin;
use ductaper::mq::upd_redact_stream::update_redact_stream;
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
}
