use crate::env_vars::Cfg;
use async_nats::jetstream::consumer::pull::Config as PullConfig;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::stream::{Config, DiscardPolicy, RetentionPolicy};
use async_nats::jetstream::Context;
use async_nats::ConnectOptions;
use futures::StreamExt;
use std::time::Duration;
use tracing::{debug, error, info};

pub struct RedactConsumer {
    pub client: async_nats::Client,
    pub jetstream: Context,
    pub consumer: Option<Consumer<async_nats::jetstream::consumer::pull::Config>>,
}

impl RedactConsumer {
    async fn serve_loop(&self, consumer: &Consumer<PullConfig>) {
        match consumer.fetch().max_messages(1).messages().await {
            Ok(mut messages) => {
                while let Some(Ok(message)) = messages.next().await {
                    if let Err(e) = message.ack().await {
                        error!("Ack failed: {}", e);
                    }
                    debug!("Got message: {:?}", message.payload);
                    if *message.payload == *b"\0" {
                        break;
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
    }

    pub async fn serve(&self) {
        if let Some(consumer) = &self.consumer {
            loop {
                self.serve_loop(consumer).await;
            }
        } else {
            error!("Consumer not found");
        }
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

        self.consumer = Some(
            match stream
                .get_or_create_consumer(
                    durable_name.as_str(),
                    PullConfig {
                        durable_name: Some(durable_name.clone()),
                        filter_subject: cfg.redact_subject.clone(),
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
            Ok(stream) => {
                match stream.get_info().await {
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
                }
            }
            Err(err) => {
                error!("Failed to get or create stream: {}", err);
            }
        }
    }

    fn get_stream_cfg(cfg: &Cfg, subjects: &mut Vec<String>) -> Config {
        async_nats::jetstream::stream::Config {
            name: cfg.queue_stream.to_string(),
            subjects: subjects.clone(),
            max_age: Duration::from_secs(cfg.queue_stream_max_age),
            retention: RetentionPolicy::Limits,
            discard: DiscardPolicy::Old,
            ..Default::default()
        }
    }

    pub async fn new(cfg: &Cfg) -> Self {
        let client = ConnectOptions::new()
            .retry_on_initial_connect() // keep retrying
            .reconnect_delay_callback(|_try| Duration::from_secs(4))
            .connect(&cfg.nats_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to NATS: {}", e);
                std::io::Error::other(e)
            })
            .unwrap();

        let jetstream = async_nats::jetstream::new(client.clone());

        RedactConsumer {
            client,
            jetstream,
            consumer: None,
        }
    }
}
