use crate::connector::Connector;
use async_nats::{Client, HeaderMap};
use bytes::Bytes;
use tracing::{debug, error};

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
        data: Vec<u8>,
        headers: Option<HeaderMap>,
    ) -> bool {
        let payload = Bytes::from(data);
        debug!("Publish {} <- {:?}", subject, payload);

        let result = if let Some(headers) = headers {
            self.nats
                .publish_with_headers(subject, headers, payload)
                .await
        } else {
            self.nats.publish(subject, payload).await
        };
        if let Err(e) = &result {
            error!("Publish error: {}", e);
        }

        result.is_ok()
    }
}
