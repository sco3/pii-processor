use crate::mq::stream_admin::StreamAdmin;
use async_nats::jetstream::stream::Config;
use tracing::{error, info};

impl StreamAdmin {
    /// updates existing stream
    /// # Errors
    ///
    /// This function returns a `Box<dyn std::error::Error + Send + Sync>` in the following cases:
    ///
    /// * **Failed to get or create the stream:** If the initial attempt to retrieve
    ///   an existing stream or create a new one fails (e.g., due to network issues,
    ///   invalid stream configuration, or NATS server problems). This error originates
    ///   from the `self.jetstream.get_or_create_stream().await` call.
    /// * **Failed to get stream information after creation/retrieval:** If the stream
    ///   was successfully obtained or created, but then fetching its detailed
    ///   configuration information immediately afterwards fails. This error comes from
    ///   the `stream.get_info().await` call within the success path of `get_or_create_stream`.
    /// * **Failed to update the stream:** If, after successfully obtaining the stream's
    ///   configuration and merging subjects, the subsequent call to update the stream
    ///   on the NATS server fails. This error originates from the
    ///   `self.jetstream.update_stream().await` call.
    pub async fn check_stream(
        &self,
        stream_name: String,
        subjects: Vec<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // ideally we need limits policy, for now use policy of existing stream
        let mut create: bool = false;
        let mut update = false;
        let mut stream_config = Config {
            name: stream_name.clone(),
            subjects: subjects.clone(), // initial value
            ..Default::default()
        };
        // get stream
        match self.jetstream.get_stream(&stream_name).await {
            Ok(s) => match s.get_info().await {
                Ok(info) => {
                    stream_config = info.config;
                    for subject in subjects.clone() {
                        if !stream_config.subjects.contains(&subject) {
                            update = true;
                        }
                    }
                    if !update {
                        info!(
                            "Stream {} with subjects: {:?} is ready.",
                            stream_name, subjects
                        );
                        return Ok(());
                    }
                }
                Err(e) => {
                    error!(
                        "Failed to get stream info: {:?}, {}",
                        e, "using defaults for stream update.",
                    );
                }
            },
            Err(e) => {
                create = true;
                info!("Stream not found {:?}, trying create it.", e);
            }
        }
        if create {
            info!("Create stream {stream_name}");
            match self.jetstream.create_stream(stream_config.clone()).await {
                Ok(_) => {
                    info!("Stream created");
                    return Ok(());
                }
                Err(e) => {
                    error!("Failed to create stream info: {}", e);
                    return Err(Box::new(e));
                }
            }
        }
        if update {
            info!("Update stream {stream_name}");
            return self.update_jetstream(subjects, &mut stream_config).await;
        }
        Ok(())
    }
}
