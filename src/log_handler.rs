use async_nats::jetstream::Message;
use async_trait::async_trait;

#[async_trait]
pub trait LogHandler {
    async fn handle(&mut self, arg: Message) -> bool;
    
}
