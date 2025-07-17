use crate::config::env_vars::Cfg;
use crate::mq::redact_consumer::RedactConsumer;
use crate::mq::stream_admin::StreamAdmin;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::consumer::pull::Config;
use std::error::Error;

use tracing::{debug, error, info};
impl RedactConsumer {
    /// Subscribe to messages.
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The subject is empty.
    /// - Fails to get the stream.
    /// - Durable name is too long (more than 64 characters).
    /// - Creating or getting the consumer fails.
    pub async fn subscribe(
        &self,
        cfg: &Cfg,
    ) -> Result<Consumer<Config>, Box<dyn Error + Send + Sync>> {
        {
            let subj = StreamAdmin::get_full_subject(cfg, &cfg.redact_subject);
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
            let durable_name = format!("consumer_{durable_safe}");
            if durable_name.len() > 64 {
                return Err("Durable name too long".into());
            }

            match stream.get_consumer(&durable_name).await {
                Ok(c) => Ok(c),
                Err(_) => stream
                    .create_consumer(Config {
                        durable_name: Some(durable_name),
                        filter_subjects: vec![subj],
                        ..Default::default()
                    })
                    .await
                    .map_err(Into::into),
            }
        }
    }
}
