use crate::mq::stream_admin::StreamAdmin;
use async_nats::jetstream::stream::Config;
use std::error::Error;
use tracing::{debug, error, info};

impl StreamAdmin {
    /// Updates the `JetStream` stream with merged subjects.
    ///
    /// This function attempts to update the given `stream_config` on the NATS server.
    /// It performs two consecutive updates:
    /// - The first to retrieve the latest config state.
    /// - The second after merging new subjects to persist the updated config.
    ///
    /// # Errors
    ///
    /// Returns `Err(Box<dyn Error + Send + Sync>)` if:
    /// - The initial call to `update_stream` fails.
    /// - The second update after subject merge also may fail.
    pub async fn update_jetstream(
        &self,
        subjects: Vec<String>,
        stream_config: &mut Config,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        // update stream
        match self.jetstream.update_stream(stream_config.clone()).await {
            Ok(info) => {
                let upd_subjects = Self::merge_unique(
                    &subjects,
                    &info.config.subjects, //
                );
                info!(
                    "Subjects required: \t\t\t {:?} \t\t\t found: {:?}",
                    subjects, info.config.subjects,
                );

                stream_config.subjects = upd_subjects;

                match self.jetstream.update_stream(stream_config).await {
                    Ok(info) => {
                        debug!("Stream updated: {:?}", info);
                        let name = info.config.name;
                        info!("Stream {name} updated with {subjects:?}");
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
        }
    }
}
