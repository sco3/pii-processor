use crate::connector::Connector;
use async_nats::Client;
use bytes::Bytes;
use tracing::error;

pub struct Publisher {
    nats: Box<Client>,
}

impl Publisher {
    pub fn new(connector: &Connector) -> Self {
        let nats = connector.get();
        Publisher { nats }
    }
    pub async fn publish(&self, subject: String, data: Bytes) -> bool {
        match self.nats.publish(subject, data).await {
            Ok(_) => true,
            Err(e) => {
                error!("Publish error: {:?}", e);
                false
            }
        }
    }
}
