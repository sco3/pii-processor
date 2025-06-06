use crate::env_vars::Cfg;

use crate::connector::Connector;
use crate::log_handler::LogHandler;
use async_nats::jetstream::consumer::pull::Config as PullConfig;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::stream::{Config, DiscardPolicy, RetentionPolicy};
use async_nats::jetstream::Context;
use futures::StreamExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tracing::{debug, error, info};

pub struct RedactConsumer {
    pub client: async_nats::Client,
    pub jetstream: Context,
    pub consumer: Option<Consumer<async_nats::jetstream::consumer::pull::Config>>,
    pub run: Arc<AtomicBool>,
    pub handler: Arc<Mutex<dyn LogHandler + Send + Sync>>,
    pub subject: Option<String>,
}

impl RedactConsumer {
    async fn serve_loop(&mut self, consumer: &Consumer<PullConfig>) {
        match consumer.fetch().max_messages(1).messages().await {
            Ok(mut messages) => {
                while let Some(Ok(message)) = messages.next().await {
                    if let Err(e) = message.ack().await {
                        error!("Ack failed: {}", e);
                    }
                    debug!("Got message: {:?}", message.payload);
                    let mut handler_guard = self.handler.lock().unwrap();
                    handler_guard.handle(message);
                    
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
    }
    pub fn get_run_flag_clone(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.run)
    }

    pub async fn serve(&mut self) {
        let consumer_option = self.consumer.take();
        if consumer_option.is_none() {
            error!("Consumer not found");
            return;
        }
        let consumer = consumer_option.unwrap();

        while self.run.load(Ordering::Relaxed) {
            self.serve_loop(&consumer).await;
        }

        self.consumer = Some(consumer);
    }
    pub async fn subscribe(&mut self, cfg: &Cfg) {
        let stream = match self.jetstream.get_stream(&cfg.queue_stream).await {
            Ok(stream) => stream,
            Err(err) => {
                error!("Failed to get stream: {}", err);
                return;
            }
        };

        let durable_name = format!("consumer_{}", cfg.redact_subject);
        info!("Creating consumer with durable name: {}", durable_name);

        let subj = cfg.redact_subject.clone();
        self.subject = Some(subj.clone());
        self.consumer = Some(
            match stream
                .get_or_create_consumer(
                    durable_name.as_str(),
                    PullConfig {
                        durable_name: Some(durable_name.clone()),
                        filter_subject: subj,
                        ..Default::default()
                    },
                )
                .await
            {
                Ok(consumer) => consumer,
                Err(err) => {
                    error!("Failed to get or create consumer: {}", err);
                    return;
                }
            },
        );
    }

    pub async fn update_stream(&self, cfg: &Cfg) {
        let mut subjects = vec![cfg.redact_subject.clone(), "test".to_string()];
        let mut stream_config = Self::get_stream_cfg(cfg, &mut subjects);
        match self
            .jetstream
            .get_or_create_stream(stream_config.clone())
            .await
        {
            Ok(stream) => match stream.get_info().await {
                Ok(info) => {
                    for existing_subject in info.config.subjects {
                        if !subjects.contains(&existing_subject) {
                            subjects.push(existing_subject);
                        }
                    }
                    stream_config.subjects = subjects;

                    match self.jetstream.update_stream(stream_config).await {
                        Ok(updated) => {
                            debug!("Stream updated: {:?}", updated);
                        }
                        Err(err) => {
                            error!("Failed to update stream: {}", err);
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to get stream info: {}", err);
                }
            },
            Err(err) => {
                error!("Failed to get or create stream: {}", err);
            }
        }
    }

    fn get_stream_cfg(cfg: &Cfg, subjects: &mut [String]) -> Config {
        async_nats::jetstream::stream::Config {
            name: cfg.queue_stream.to_string(),
            subjects: subjects.to_vec(),
            max_age: Duration::from_secs(cfg.queue_stream_max_age),
            retention: RetentionPolicy::Limits,
            discard: DiscardPolicy::Old,
            ..Default::default()
        }
    }

    pub async fn new(
        connector: Connector, //
        handler: Arc<Mutex<dyn LogHandler + Send + Sync>>,
    ) -> Self {
        let client = *connector.get();

        let jetstream = async_nats::jetstream::new(client.clone());

        RedactConsumer {
            client,
            jetstream,
            consumer: None,
            run: Arc::new(AtomicBool::new(true)),
            handler,
            subject: None,
        }
    }
}
