use crate::env_vars::Cfg;
use async_nats::ConnectOptions;
use async_nats::jetstream::Context;
use async_nats::jetstream::stream::RetentionPolicy;
use std::time::Duration;
use tracing::{debug, error};

pub struct RedactConsumer {
    pub client: async_nats::Client,
    pub jetstream: Context,
}

impl RedactConsumer {
    pub async fn update_stream(&self, cfg: &Cfg) {
        let mut subjects = vec![cfg.redact_subject.clone()];
        let mut stream_config = async_nats::jetstream::stream::Config {
            name: cfg.queue_stream.to_string(),
            subjects: subjects.clone(),
            max_age: Duration::from_secs(cfg.queue_stream_max_age),
            retention: RetentionPolicy::Limits,
            ..Default::default()
        };
        let _stream = match self
            .jetstream
            .get_or_create_stream(stream_config.clone())
            .await
        {
            Ok(stream) => {
                let _info = match stream.get_info().await {
                    Ok(info) => {
                        for existing_subject in info.config.subjects {
                            if !subjects.contains(&existing_subject) {
                                subjects.push(existing_subject);
                            }
                        }
                        stream_config.subjects = subjects;

                        let _upd = match self.jetstream.update_stream(stream_config).await {
                            Ok(updated) => {
                                debug!("Stream updated: {:?}", updated);
                            }
                            Err(err) => {
                                error!("Failed to update stream: {}", err);
                                return;
                            }
                        };
                    }
                    Err(err) => {
                        error!("Failed to get stream info: {}", err);
                        return;
                    }
                };
            }
            Err(err) => {
                error!("Failed to get or create stream: {}", err);
                return;
            }
        };
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

        RedactConsumer { client, jetstream }
    }
}
