mod common;

use crate::common::init_cfg::get_test_cfg;
use async_nats::jetstream::Message;
use async_trait::async_trait;
pub use common::init_logging::init_tracing;
use ductaper::connector::Connector;
use ductaper::log_handler::LogHandler;
use ductaper::publisher::Publisher;
use ductaper::redact_consumer::RedactConsumer;
use reqwest::StatusCode;
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};
use tokio::sync::broadcast::{channel as broadcast_channel, Sender as BroadcastSender};

use tokio::sync::Mutex;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::{debug, info};

struct DummyHandler {
    count_snd: BroadcastSender<()>,
}

#[async_trait]
impl LogHandler for DummyHandler {
    async fn handle(&mut self, msg: Message) -> bool {
        let start = std::str::from_utf8(&msg.payload)
            .unwrap_or("0")
            .parse::<u128>()
            .unwrap_or_default();
        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros()
            - start;

        debug!("Message arrive time: {} µs", duration);

        if let Err(e) = self.count_snd.send(()) {
            panic!("Send problem : {}", e);
        }
        true
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
        let (count_snd, mut count_rcv1) = broadcast_channel::<()>(8);
        let mut count_rcv2 = count_snd.subscribe();
        let dummy_handler = DummyHandler { count_snd };
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
        let total_count = AtomicI64::new(0);
        let _ = tokio::join!(
            async {
                // test duration
                sleep(TokioDuration::from_millis(42)).await;
                info!("Exit sleep");
                run.store(false, Ordering::Relaxed);
                info!("Exit time limit");
            },
            async {
                for _ in 0..2 {
                    let ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_micros();
                    publisher.publish(subj.clone(), ts.to_string().into()).await;
                }
                info!("Exit publish");
            },
            async {
                while run.load(Ordering::Relaxed) {
                    if count_rcv1.try_recv().is_ok() {
                        total_count.fetch_add(1, Ordering::Relaxed);
                        info!("Counting1: {}", total_count.load(Ordering::Relaxed));
                    } else {
                        //info!("Not received: {}", run.load(Ordering::Relaxed));
                    }
                    sleep(TokioDuration::from_nanos(1)).await;
                }
            },
            async {
                while run.load(Ordering::Relaxed) {
                    if count_rcv2.try_recv().is_ok() {
                        total_count.fetch_add(1, Ordering::Relaxed);
                        info!("Counting2: {}", total_count.load(Ordering::Relaxed));
                    } else {
                        //info!("Not received: {}", run.load(Ordering::Relaxed));
                    }
                    sleep(TokioDuration::from_nanos(1)).await;
                }
            },
            consumer.serve(),
        );

        info!("Count: {}", total_count.load(Ordering::Relaxed));
        assert_eq!(4, total_count.load(Ordering::Relaxed));
    }
}
