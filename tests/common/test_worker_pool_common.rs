use async_channel::bounded;
use async_nats::jetstream::Message;
use ductaper::worker_pool::WorkerPool;
use reqwest::StatusCode;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

use tracing::info;

use ductaper::util::logging::init_tracing;

use crate::common::init_cfg::get_test_cfg;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::mq::connector::Connector;

use crate::common::dummy_saver::DummySaver;
use ductaper::llm_work::llm_caller::LLmCaller;
use ductaper::llm_work::prompt::read_prompt;
use ductaper::mq::publisher::Publisher;
use ductaper::mq::redact_consumer::RedactConsumer;
use ductaper::worker_pool::event_counter::MinuteCounter;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct TestPoolResult {
    pub run_flag: Arc<AtomicBool>,
    pub pool: WorkerPool,
    pub container: ContainerAsync<GenericImage>,
}
#[allow(dead_code)]
pub async fn test_pool(payload: Vec<u8>) -> TestPoolResult {
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
        "http:localhost:4000/chat/completions",
        "haiku",
        Some("sk-1234".to_string()),
    ));

    let system_prompt = read_prompt(&"data/system_prompt.txt".to_string());
    let processor = LlmLogProcessor::new(
        caller, //
        system_prompt,
        "haiku".to_string(),
        Arc::new(DummySaver::new()),
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
    if let Err(e) = consumer.subscribe(&cfg).await {
        panic!("Subscription: {}", e);
    }
    let subject = consumer.subject.clone().unwrap_or_default();
    info!("Subject: {}", subject);

    let run_flag = consumer.run_flag.clone();
    tokio::spawn(async move {
        consumer.serve().await;
    });

    let publisher = Publisher::new(&conn);

    let str = String::from_utf8(payload.clone()).expect("Should be valid utf8 text");
    info!(
        "Publish:\n\nnats pub {} '{}' -s nats://localhost:{}\n\n",
        subject, str, port
    );
    // empty payload
    publisher
        .publish(
            subject.clone(), //
            payload,
            None,
        )
        .await;

    info!(
        "Publish:\n\nnats pub {} '{}' -s nats://localhost:{}\n\n",
        subject, "something", port
    );

    TestPoolResult {
        run_flag,
        pool,
        container,
    }
}
