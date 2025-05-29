use crate::mq::ack::Ack;
use async_nats::jetstream::Message;
use async_trait::async_trait;
use std::error::Error;
/// wrapper for nats message
pub struct NatsAck {
    /// nats message to acknowledge
    pub msg: Message,
}

impl NatsAck {
    /// constructor
    pub fn from(msg: &Message) -> Self {
        NatsAck { msg: msg.clone() }
    }
}

#[async_trait]
impl Ack for NatsAck {
    /// calls ack on wrapped message
    async fn ack(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.msg.ack().await
    }
}
