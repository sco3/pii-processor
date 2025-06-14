use crate::config::env_vars::Cfg;
use std::process::exit;

use crate::llm_work::preview::preview_bytes;
use crate::mq::admin::StreamAdmin;
use crate::mq::connector::Connector;
use crate::util::exit_codes::ExitCode;
use async_channel::Sender;
use async_nats::jetstream::consumer::pull::Config as PullConfig;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::{Context, Message};
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info};

pub struct RedactConsumer {
    pub client: async_nats::Client,
    pub jetstream: Context,
    pub consumer: Option<Arc<Consumer<PullConfig>>>,
    pub run_flag: Arc<AtomicBool>,
    pub sender: Sender<Message>,
}

impl RedactConsumer {
    pub fn get_run_flag(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.run_flag)
    }

    pub fn stop(&self) {
        info!("Stop consumer");
        let flag = self.get_run_flag();
        flag.store(false, Ordering::Relaxed);
        self.sender.close();
        info!("Consumer stopped");
    }
    pub async fn start(&self, cfg: &Cfg) {
        let consumer = match self.subscribe(cfg).await {
            Ok(c) => c,
            Err(e) => {
                error!("Subscription failed: {}", e);
                exit(ExitCode::NatsError.code());
            }
        };
        let run_flag = Arc::clone(&self.run_flag);
        let sender = self.sender.clone();
        let consumer = consumer;

        tokio::spawn(async move {
            Self::serve(run_flag, sender, &consumer).await;
        });
    }

    async fn fetch(sender: &Sender<Message>, consumer: &Consumer<PullConfig>) {
        match consumer.fetch().max_messages(1).messages().await {
            Ok(mut messages) => {
                while let Some(Ok(message)) = messages.next().await {
                    debug!("Got message: {:?}", preview_bytes(&message.payload),);
                    if let Err(e) = sender.send(message).await {
                        error!("Failed to send message to channel: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
    }

    pub async fn serve(
        run_flag: Arc<AtomicBool>,
        sender: Sender<Message>,
        consumer: &Consumer<PullConfig>,
    ) {
        info!("Start serving");
        while run_flag.load(Ordering::Relaxed) {
            Self::fetch(&sender, consumer).await;
            sleep(Duration::from_micros(1)).await
        }
        info!("Exit serve");
    }

    // pub fn get_full_subject(cfg: &Cfg) -> String {
    //     format!("{}.{}.{}", cfg.tenant, cfg.application, cfg.redact_subject)
    // }

    pub async fn subscribe(
        &self,
        cfg: &Cfg,
    ) -> Result<Consumer<PullConfig>, Box<dyn std::error::Error>> {
        let subj = StreamAdmin::get_full_subject(cfg, cfg.redact_subject.clone());
        info!("Attempt to subscribe to: {}", subj);

        if subj.is_empty() {
            return Err("Empty subject".into());
        }

        let stream = self
            .jetstream
            .get_stream(&cfg.queue_stream)
            .await
            .map_err(|e| {
                error!("Failed to get stream: {}", e);
                e
            })?;

        debug!("Found existing stream: {}", &cfg.queue_stream);

        let durable_safe = subj.replace('.', "_");
        let durable_name = format!("consumer_{}", durable_safe);
        if durable_name.len() > 64 {
            return Err("Durable name too long".into());
        }

        info!("Consumer: {} subject: {}", durable_name, subj);

        let consumer = stream
            .get_or_create_consumer(
                &durable_name,
                PullConfig {
                    durable_name: Some(durable_name.clone()),
                    filter_subjects: vec![subj.clone()],
                    ..Default::default()
                },
            )
            .await
            .map_err(|e| {
                error!("Failed to create consumer: {}", e);
                e
            })?;

        Ok(consumer)
    }

    pub async fn new(connector: &Connector, sender: Sender<Message>) -> Self {
        let client = *connector.get();
        let jetstream = async_nats::jetstream::new(client.clone());

        RedactConsumer {
            client,
            jetstream,
            consumer: None,
            run_flag: Arc::new(AtomicBool::new(true)),
            sender,
        }
    }
}
