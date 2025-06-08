use crate::connector::Connector;
use async_nats::{Client, HeaderMap};
use bytes::Bytes;
use tracing::debug;

pub struct Publisher {
    nats: Box<Client>,
}

impl Publisher {
    pub fn new(connector: &Connector) -> Self {
        let nats = connector.get();
        Publisher { nats }
    }
    pub async fn publish(
        &self, //
        subject: String,
        data: Bytes,
        headers: Option<HeaderMap>,
    ) -> bool {
        debug!("Publish {:?} to {}", data, subject);
        if let Some(headers) = headers {
            self.nats
                .publish_with_headers(subject, headers, data)
                .await
                .is_ok()
        } else {
            self.nats.publish(subject, data).await.is_ok()
        }
    }
}
