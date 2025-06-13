use crate::config::env_vars::Cfg;
use crate::mq::connector::Connector;
use std::process::exit;

use crate::util::exit_codes::ExitCode;
use async_nats::jetstream::stream::Config;
use async_nats::jetstream::Context;
use tracing::{debug, error};

pub struct StreamAdmin {
    pub jetstream: Context,
}

impl StreamAdmin {
    pub fn new(connector: &Connector) -> Self {
        let client = *connector.get();
        let jetstream = async_nats::jetstream::new(client.clone());
        StreamAdmin { jetstream }
    }
}

impl StreamAdmin {
    pub fn get_full_subject(cfg: &Cfg, subject: String) -> String {
        format!("{}.{}.{}", &cfg.tenant, &cfg.application, subject)
    }

    pub async fn update_stream(
        &self,
        stream_name: String,
        subjects: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut subjs = Vec::new();
        subjs.extend(subjects.iter().cloned());
        let mut stream_config = Self::get_stream_cfg(stream_name, &subjs);

        match self
            .jetstream
            .get_or_create_stream(stream_config.clone())
            .await
        {
            Ok(stream) => match stream.get_info().await {
                Ok(info) => {
                    for existing_subject in info.config.subjects {
                        if !subjects.contains(&existing_subject) {
                            subjs.push(existing_subject);
                        }
                    }
                    stream_config.subjects = subjects;

                    match self.jetstream.update_stream(stream_config).await {
                        Ok(updated) => {
                            debug!("Stream updated: {:?}", updated);
                            Ok(())
                        }
                        Err(err) => {
                            error!("Failed to update stream: {}", err);
                            Err(Box::new(err))
                        }
                    }
                }
                Err(err) => {
                    error!("Failed to get stream info: {}", err);
                    Err(Box::new(err))
                }
            },
            Err(err) => {
                error!("Failed to get or create stream: {}", err);
                Err(Box::new(err))
            }
        }
    }

    fn get_stream_cfg(stream: String, subjects: &Vec<String>) -> Config {
        async_nats::jetstream::stream::Config {
            name: stream,
            subjects: subjects.clone(),
            ..Default::default()
        }
    }
    pub async fn update_redact_stream(&self, cfg: &Cfg, exit_app: bool) {
        if let Err(e) = self
            .update_stream(
                cfg.queue_stream.clone(), //
                vec![StreamAdmin::get_full_subject(
                    &cfg,
                    cfg.redact_subject.clone(),
                )],
            )
            .await
        {
            if exit_app {
                error!("Cannot update stream {}", e);
                exit(ExitCode::NatsError.code());
            } else {
                panic!("Cannot update stream {}", e);
            }
        }
    }
}
