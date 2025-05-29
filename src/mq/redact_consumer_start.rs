use crate::config::env_vars::Cfg;
use crate::mq::redact_consumer::RedactConsumer;
use crate::util::exit_codes::ExitCode;
use std::process::exit;

use tokio::sync::oneshot::{Sender as OneSender, channel};
use tokio::task::JoinHandle;
use tracing::error;

/// structure for graceful stop of consumer
pub struct ConsumerStop {
    /// gracefull stop needs wait this
    pub join_handle: JoinHandle<()>,
    /// send signal with this sender to stop consumer
    pub stop_tx: OneSender<()>,
}

impl RedactConsumer {
    /// start the consumer
    pub async fn start(&self, cfg: &Cfg) -> ConsumerStop {
        let consumer = match self.subscribe(cfg).await {
            Ok(c) => c,
            Err(e) => {
                error!("Subscription failed: {}", e);
                exit(ExitCode::NatsError.code());
            }
        };

        let sender = self.sender.clone();
        let consumer = consumer;

        let (stop_tx, stop_rx) = channel::<()>();

        let join_handle = tokio::spawn(async move {
            Self::serve(stop_rx, sender, &consumer).await;
        });
        ConsumerStop {
            join_handle,
            stop_tx,
        }
    }
}
