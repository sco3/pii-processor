use crate::session_log_models::SessionLogType;
use async_trait::async_trait;

#[async_trait]
pub trait Saver: Send + Sync {
    async fn save(
        &self,
        _log: SessionLogType,
        _file_name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }
}
