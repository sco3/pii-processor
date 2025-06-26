use crate::config::env_vars::Cfg;
use crate::data::session_log_models::SessionLog;
use async_trait::async_trait;

/// saver trate (interface)
#[async_trait]
pub trait Saver: Send + Sync {
    /// save method
    async fn save(&self, _log: SessionLog, _file_name: &str) -> bool;
    /// name getter
    fn get_name(&self) -> String;

    /// lazy init of saver
    async fn init(&self, cfg: &Cfg);
}
