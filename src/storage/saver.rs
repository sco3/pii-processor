use crate::session_log_models::SessionLog;
use async_trait::async_trait;

#[async_trait]
pub trait Saver: Send + Sync {
    async fn save(&self, _log: SessionLog, _file_name: &str) -> bool {
        false
    }
}
