use async_nats::ConnectOptions;
use std::time::Duration;
use tracing::error;

pub struct RedactConsumer {
    pub client: async_nats::Client,
}

impl RedactConsumer {
    pub async fn new(nats_url: &str) -> Self {
        let client = ConnectOptions::new()
            .retry_on_initial_connect() // keep retrying
            .reconnect_delay_callback(|_try| Duration::from_secs(4))
            .connect(nats_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to NATS: {}", e);
                std::io::Error::new(std::io::ErrorKind::Other, e)
            })
            .unwrap();

        RedactConsumer { client }
    }
}
