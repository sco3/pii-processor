mod common;

use crate::common::init_cfg::get_test_cfg;
use async_channel::{bounded, Receiver, Sender};
use async_nats::jetstream::Message;
use async_trait::async_trait;

use ductaper::connector::Connector;
use ductaper::llm_work::log_handler::LogHandler;
pub use ductaper::logging::init_tracing;
use ductaper::mq::publisher::Publisher;
use ductaper::mq::redact_consumer::RedactConsumer;
use reqwest::StatusCode;
use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::{
    core::{IntoContainerPort, WaitFor}, runners::AsyncRunner,
    GenericImage,
    ImageExt,
};

use ductaper::mq::admin::StreamAdmin;
use tokio::time::sleep;
use tokio::time::Duration as TokioDuration;
use tracing::{debug, info};

struct DummyHandler {
    count_snd: Sender<()>,
    receiver: Receiver<Message>,
}

impl DummyHandler {
    pub async fn start(&mut self) {
        while let Ok(msg) = self.receiver.recv().await {
            self.handle(msg).await;
        }
        info!("Exit handler");
    }
}

#[async_trait]
impl LogHandler for DummyHandler {
    async fn handle(&mut self, msg: Message) -> bool {
        let slice = msg.payload.as_ref();
        let arr: [u8; 16] = slice.try_into().unwrap_or_default();
        let start = u128::from_be_bytes(arr);

        let duration = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_micros()
            - start;

        debug!("Message arrive time: {} µs", duration);

        if let Err(e) = self.count_snd.send(()).await {
            panic!("Send problem : {}", e);
        }
        true
    }
}

#[tokio::test]
async fn test_consumer() {
    init_tracing();
    // unsafe {
    //     std::env::remove_var("DOCKER_HOST");
    // }

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
        let (count_snd, count_rcv1) = bounded::<()>(8);
        let count_rcv2 = count_rcv1.clone();

        //let dummy = Arc::new(Mutex::new(dummy_handler));
        let (msg_send, receiver) = bounded::<Message>(1);
        let mut consumer = RedactConsumer::new(
            &conn, //
            msg_send,
        )
        .await;

        let admin = StreamAdmin::new(&conn);
        admin.update_redact_stream(&cfg, false).await;

        consumer.subscribe(&cfg).await;
        let run = consumer.get_run_flag_clone();
        let subj = consumer.subject.clone().unwrap_or_default();
        info!("Subject: {}", subj);
        let total_count = AtomicI64::new(0);

        let mut dummy_handler = DummyHandler {
            count_snd,
            receiver: receiver.clone(),
        };

        let _ = tokio::join!(
            async {
                // test duration
                sleep(TokioDuration::from_millis(42)).await;
                info!("Exit sleep");
                run.store(false, Ordering::Relaxed);
                receiver.close();
                info!("Exit time limit");
            },
            async {
                for _ in 0..2 {
                    let ts = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_micros();
                    publisher
                        .publish(
                            subj.clone(), //
                            ts.to_be_bytes().to_vec(),
                            None,
                        )
                        .await;
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
                    }
                    sleep(TokioDuration::from_nanos(1)).await;
                }
            },
            dummy_handler.start(),
            consumer.serve(),
        );

        info!("Count: {}", total_count.load(Ordering::Relaxed));
        assert_eq!(2, total_count.load(Ordering::Relaxed));
    }
}
