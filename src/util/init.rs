use async_trait::async_trait;

#[async_trait]
pub trait Init {
    async fn start(&mut self);
}
