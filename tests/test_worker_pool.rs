mod common;
use async_channel::bounded;
use async_nats::jetstream::Message;
use ductaper::worker_pool::WorkerPool;
use reqwest::StatusCode;

use tracing::{debug, info};

use common::init_logging::init_tracing;

use crate::common::init_cfg::get_test_cfg;
use ductaper::connector::Connector;
use ductaper::publisher::Publisher;
use ductaper::redact_consumer::RedactConsumer;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};

#[tokio::test]
async fn test_pool() {
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

    let port = match container.get_host_port_ipv4(4222.tcp()).await {
        Ok(p) => p,
        Err(e) => {
            panic!("Failed to get port: {}", e);
        }
    };

    info!("Container port: {port}");

    let (tx, rx) = bounded::<Message>(1);
    debug!("Tx: {:?}", tx);

    let pool = WorkerPool {
        size: 1,
        receiver: rx,
    };
    pool.start().await;
    let cfg = get_test_cfg(port);
    let conn = Connector::new(cfg.clone()).await;
    let publisher = Publisher::new(&conn);
    let consumer = RedactConsumer::new(conn, tx);
    let subject = cfg.redact_subject;
    info!("Subject: {}", subject);
}
