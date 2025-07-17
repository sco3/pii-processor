use crate::llm_work::preview::preview_bytes;
use crate::mq::connector::Connector;
use async_channel::Sender;
use async_nats::jetstream::consumer::Consumer;
use async_nats::jetstream::consumer::pull::Config as PullConfig;
use async_nats::jetstream::{Context, Message};
use futures::StreamExt;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::oneshot::Receiver as OneReceiver;

use tokio::time::sleep;
use tracing::{debug, error, info};

/// struct for nats consuming
pub struct RedactConsumer {
    /// nats client
    pub client: async_nats::Client,
    /// jest stream client
    pub jetstream: Context,
    /// consumer
    pub consumer: Option<Arc<Consumer<PullConfig>>>,
    /// the sender to send consumed messages
    /// the worker pool workers receive this in distributed mode
    pub sender: Sender<Message>,
}

impl RedactConsumer {
    /// run flag getter
    /// fetches the message from nats
    async fn fetch(sender: &Sender<Message>, consumer: &Consumer<PullConfig>) {
        match consumer.fetch().max_messages(1).messages().await {
            Ok(mut messages) => {
                while let Some(Ok(message)) = messages.next().await {
                    debug!("Got message: {:?}", preview_bytes(&message.payload),);
                    if let Err(e) = sender.send(message).await {
                        error!("Failed to send message to channel: {}", e);
                    }
                }
            }
            Err(e) => {
                error!("Failed to fetch messages: {}", e);
            }
        }
    }
    /// method to serve (consume and dispatch messages)
    pub async fn serve(
        mut stop_rx: OneReceiver<()>,
        sender: Sender<Message>,
        consumer: &Consumer<PullConfig>,
    ) {
        info!("Start consumer");

        tokio::select! {
            _ = &mut stop_rx => {
                info!("Stop consumer");
            }
            _ = async {
                    loop {
                        Self::fetch(&sender, consumer).await;
                        sleep(Duration::from_micros(1)).await;
                    }
            } =>{}
        }
        info!("Consumer stopped");
    }

    #[must_use]
    /// constructor
    pub fn new(connector: &Connector, sender: Sender<Message>) -> Self {
        let client = *connector.get();
        let jetstream = async_nats::jetstream::new(client.clone());

        RedactConsumer {
            client,
            jetstream,
            consumer: None,
            sender,
        }
    }
}
