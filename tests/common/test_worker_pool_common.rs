use async_channel::bounded;
use async_nats::jetstream::Message;
use ductaper::worker_pool::WorkerPool;
use reqwest::StatusCode;

use std::sync::Arc;

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
use ductaper::mq::redact_consumer_start::ConsumerStop;
use ductaper::mq::stream_admin::StreamAdmin;
use ductaper::mq::upd_redact_stream::update_redact_stream;
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{IntoContainerPort, WaitFor},
    runners::AsyncRunner,
};

#[allow(unused_variables)]
#[allow(dead_code)]
pub struct TestPoolResult {
    pub pool: WorkerPool,
    pub container: ContainerAsync<GenericImage>,
    pub consumer_stop: ConsumerStop,
}
#[allow(dead_code)]
pub async fn test_pool(payload: Vec<u8>) -> TestPoolResult {
    init_tracing();
    let cfg = get_test_cfg(0);

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
            panic!("Failed to get port: {e}");
        }
    };

    info!("Nat server: nats://localhost:{port}");

    let (tx, rx) = bounded::<Message>(1);

    let caller = Arc::new(LLmCaller::new(
        "http:localhost:4000/chat/completions",
        "haiku",
        Some(&"sk-1234".to_string()),
        false,
        0,
        &cfg,
    ));

    let system_prompt = read_prompt("data/system_prompt.txt", false);
    let processor = LlmLogProcessor::new(
        caller, //
        system_prompt,
        "haiku",
        Arc::new(DummySaver::new()),
    );
    let shared_processor = Arc::new(processor);

    let mut pool = WorkerPool {
        size: 1,
        receiver: rx,
        llm_log_processor: shared_processor,
        handlers: Vec::new(),
    };
    pool.start();
    let cfg = get_test_cfg(port);
    let conn = Connector::new(&cfg, None).await;
    let admin = StreamAdmin::new(&conn);
    let consumer = RedactConsumer::new(&conn, tx);
    update_redact_stream(&admin, &cfg).await;
    if let Err(e) = consumer.subscribe(&cfg).await {
        panic!("Subscription: {e}");
    }
    let subject = StreamAdmin::get_full_subject(&cfg, &cfg.redact_subject);
    info!("Subject: {}", subject);

    let consumer_stop = consumer.start(&cfg).await;

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
        pool,
        container,
        consumer_stop,
    }
}
