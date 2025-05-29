use async_trait::async_trait;
use std::error::Error;

/// ack trate (interface)
#[async_trait]
pub trait Ack: Send + Sync {
    /// acknowledge
    async fn ack(&self) -> Result<(), Box<dyn Error + Send + Sync>>;
}
