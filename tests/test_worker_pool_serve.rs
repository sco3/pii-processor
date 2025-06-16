use async_nats::HeaderMap;
use axum::http::StatusCode;
use ductaper::util::logging::init_tracing;
use std::sync::Arc;
use std::time::Duration;
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
use crate::common::init_cfg::get_test_cfg;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::mq::admin::StreamAdmin;
use ductaper::mq::connector::Connector;
use ductaper::mq::publisher::Publisher;
use ductaper::mq::redact_consumer::RedactConsumer;
use ductaper::mq::session_log_header::SESSION_LOG_HEADER;
use ductaper::mq::upd_redact_stream::update_redact_stream;
use ductaper::worker_pool::WorkerPool;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::debug;

#[tokio::test]
async fn test_worker_pool_serve() {
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

        let cfg = get_test_cfg(port);
        let resp = std::fs::read_to_string("tests/data/response.json").unwrap();

        let proc = LlmLogProcessor::new(
            Arc::new(DummyCaller::new(Some(&resp))),
            "Help if you can.".to_string(),
            "nova".to_string(),
            Arc::new(DummySaver::new()),
        );

        let (tx, rx) = async_channel::bounded(1);

        let _wp = WorkerPool {
            size: 1,
            receiver: rx,
            llm_log_processor: Arc::new(proc),
            handlers: Vec::new(),
        };
        debug!("ok");
        sleep(TokioDuration::from_millis(1)).await;

        let conn = Connector::new(cfg.clone()).await;

        let consumer = RedactConsumer::new(
            &conn, //
            tx,
        )
        .await;

        let admin = StreamAdmin::new(&conn);
        update_redact_stream(&admin, &cfg).await;

        if let Err(e) = consumer.subscribe(&cfg).await {
            panic!("Subscribe fail: {}", e);
        }

        let publisher = Publisher::new(&conn);
        let test_log = std::fs::read(
            "tests/data/worker-pool-test.json", //
        )
        .unwrap();
        let full_subj = StreamAdmin::get_full_subject(
            &cfg, //
            cfg.redact_subject.clone(),
        )
        .clone();
        let mut headers = HeaderMap::new();
        headers.insert(SESSION_LOG_HEADER, "out.json");

        publisher.publish(full_subj, test_log, Some(headers)).await;
        sleep(Duration::from_secs(2)).await;
    }
}
