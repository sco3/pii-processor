mod common;

use async_channel::bounded;
use async_nats::jetstream::Message;
use ductaper::worker_pool::WorkerPool;
use reqwest::StatusCode;
use std::fs;
use std::sync::atomic::Ordering;
use std::sync::Arc;

use bytes::Bytes;
use std::time::Duration;
use tracing::{debug, info};

use ductaper::init_logging::init_tracing;

use crate::common::init_cfg::get_test_cfg;
use ductaper::connector::Connector;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;

use ductaper::llm_caller::LLmCaller;
use ductaper::llm_work::preview::preview;
use ductaper::llm_work::prompt::prompt;
use ductaper::publisher::Publisher;
use ductaper::redact_consumer::RedactConsumer;
use ductaper::worker_pool::event_counter::MinuteCounter;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use tokio::time::sleep;

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

    info!("Nat server: nats://localhost:{port}");

    let (tx, rx) = bounded::<Message>(1);

    let caller = Arc::new(LLmCaller::new(
        "http:localhost:4000",
        "haiku",
        Some("sk-1234".to_string()),
    ));

    let system_prompt = prompt(&"data/system_prompt.txt".to_string());
    let processor = LlmLogProcessor::new(
        caller, //
        system_prompt,
        "haiku".to_string(),
    );
    let shared_processor = Arc::new(processor);

    let pool = WorkerPool {
        size: 1,
        receiver: rx,
        counter: MinuteCounter::new(),
        llm_log_processor: shared_processor,
    };
    pool.start().await;
    let cfg = get_test_cfg(port);
    let conn = Connector::new(cfg.clone()).await;

    let mut consumer = RedactConsumer::new(&conn, tx).await;
    consumer.update_stream(&cfg).await;
    consumer.subscribe(&cfg).await;
    let subject = consumer.subject.clone().unwrap_or_default();
    info!("Subject: {}", subject);

    let run_flag = consumer.run_flag.clone();
    tokio::spawn(async move {
        consumer.serve().await;
    });

    let publisher = Publisher::new(&conn);
    let empty_payload = "[]";
    info!(
        "Publish:\n\nnats pub {} '{}' -s nats://localhost:{}\n\n",
        subject, empty_payload, port
    );
    // empty payload
    publisher
        .publish(
            subject.clone(), //
            empty_payload.into(),
            None,
        )
        .await;
    // wrong payload
    publisher
        .publish(
            subject.clone(), //
            "{}".into(),
            None,
        )
        .await;
    //////////////////////////////////////////////////////////////////

    test_session_log_file(subject, publisher).await;
    ///////////////////////////////////////////////////////////////////
    sleep(Duration::from_millis(42)).await;
    info!("Stop");
    run_flag.store(false, Ordering::Relaxed);
}

async fn test_session_log_file(subject: String, publisher: Publisher) {
    let path = "tests/data/worker-pool-test.json";
    let file_content = fs::read(path) //
        .expect("Failed to read example_new_fields.log");

    let preview: Bytes = preview(&file_content);
    debug!("Session log preview: {:?}", preview);

    publisher
        .publish(
            subject, //
            file_content,
            None,
        )
        .await;
}
