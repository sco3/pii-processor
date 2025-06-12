use crate::env_vars::Cfg;
use async_nats::{Client, ConnectOptions};
use std::time::Duration;
use tracing::{error, info};

pub struct Connector {
    nats: Box<Client>,
}

impl Connector {
    pub async fn new(cfg: Cfg) -> Self {
        info!("Connect to nats: {}", cfg.nats_url);
        let nats = ConnectOptions::new()
            .retry_on_initial_connect() // keep retrying
            .reconnect_delay_callback(|_try| Duration::from_secs(4))
            .connect(&cfg.nats_url)
            .await
            .map_err(|e| {
                error!("Failed to connect to NATS: {}", e);
                std::io::Error::other(e)
            })
            .unwrap();
        Connector {
            nats: Box::new(nats),
        }
    }
    pub fn get(&self) -> Box<Client> {
        self.nats.clone()
    }
}
