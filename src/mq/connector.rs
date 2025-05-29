/// nats connection functionality
use crate::config::env_vars::Cfg;
use crate::probe::toggle::Toggle;
use async_nats::{Client, ConnectOptions};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

/// nats connector structure
pub struct Connector {
    nats: Box<Client>,
}
/// connector methods
impl Connector {
    /// constructor
    pub async fn new(cfg: &Cfg, ready_toggle: Option<&Toggle>) -> Self {
        info!("Connect to nats: {}", cfg.nats_url);
        loop {
            let retry = 4;
            match ConnectOptions::new()
                .retry_on_initial_connect() // keep retrying
                .reconnect_delay_callback(|_try| Duration::from_secs(4))
                .connect(&cfg.nats_url)
                .await
                .map_err(|e| {
                    error!("Failed to connect to NATS: {}", e);
                    std::io::Error::other(e)
                }) {
                Ok(nats) => {
                    if let Some(toggle) = ready_toggle {
                        toggle.set_ready(true);
                    }
                    return Connector {
                        nats: Box::new(nats),
                    };
                }
                Err(e) => {
                    if let Some(toggle) = ready_toggle {
                        toggle.set_ready(false);
                    }
                    warn!("Failed to connect to NATS: {e} retry in {retry} s");

                    sleep(Duration::from_secs(retry)).await;
                }
            }
        }
    }
    /// clone instance
    #[must_use]
    pub fn get(&self) -> Box<Client> {
        self.nats.clone()
    }
}
