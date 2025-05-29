use async_trait::async_trait;
/// trait for starter
#[async_trait]
pub trait Init {
    /// methods start application
    async fn start(&mut self);
}
