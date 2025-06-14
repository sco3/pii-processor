use crate::config::env_vars::Cfg;
use crate::mq::connector::Connector;

use async_nats::jetstream::Context;
use async_nats::jetstream::stream::Config;
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
        let mut merged_subjects = subjects.clone();

        let mut stream_config = Config {
            name: stream_name.clone(),
            subjects: subjects.clone(), // initial value
            ..Default::default()
        };

        match self
            .jetstream
            .get_or_create_stream(stream_config.clone())
            .await
        {
            Ok(stream) => match stream.get_info().await {
                Ok(info) => {
                    for existing_subject in info.config.subjects {
                        if !merged_subjects.contains(&existing_subject) {
                            merged_subjects.push(existing_subject);
                        }
                    }

                    stream_config.subjects = merged_subjects;

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
}
