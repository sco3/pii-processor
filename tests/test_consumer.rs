mod common;

use crate::common::init_cfg::get_test_cfg;
use async_nats::jetstream::Message;
pub use common::init_logging::init_tracing;
use ductaper::connector::Connector;
use ductaper::log_handler::LogHandler;
use ductaper::publisher::Publisher;
use ductaper::redact_consumer::RedactConsumer;
use reqwest::StatusCode;
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use tokio;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::{debug, info};

struct DummyHandler {
    count: i32,
}

impl LogHandler for DummyHandler {
    fn handle(&mut self, msg: Message) -> bool {
        debug!("Message: {:?}", msg);
        self.count += 1;
        true
    }
    fn cnt(&self) -> i32 {
        self.count
    }
}

#[tokio::test]
async fn test_consumer() {
    init_tracing();
    unsafe {
        std::env::remove_var("DOCKER_HOST");
    }

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
        let conn = Connector::new(cfg.clone()).await;
        let publisher = Publisher::new(&conn);
        let dummy_handler = DummyHandler { count: 0 };
        let dummy = Arc::new(Mutex::new(dummy_handler));

        let mut consumer = RedactConsumer::new(
            conn, //
            dummy,
        )
        .await;
        consumer.update_stream(&cfg).await;
        consumer.subscribe(&cfg).await;
        let run = consumer.get_run_flag_clone();
        let subj = consumer.subject.clone().unwrap_or_default();
        let _ = tokio::join!(
            async {
                publisher.publish(subj, "asdf".into()).await;
                sleep(TokioDuration::from_secs(2)).await;
                run.store(false, Ordering::Relaxed);
            },
            consumer.serve(),
        );
        let handler_guard = consumer.handler.lock().unwrap();

        info!("Count: {}", handler_guard.cnt());
        assert_eq!(1, handler_guard.cnt());
    }
}
