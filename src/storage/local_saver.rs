use crate::config::env_vars::Cfg;
use crate::data::session_log_models::SessionLog;
use crate::storage::saver::Saver;
use async_trait::async_trait;
use std::fs;
use std::path::Path;
use tracing::{debug, error};

/// structure to save files locally
pub struct LocalSaver {
    /// directory
    pub dir: String,
}
impl LocalSaver {
    /// constructor
    #[must_use]
    pub fn new(dir: &str) -> Self {
        LocalSaver {
            dir: dir.to_string(),
        }
    }
}

#[async_trait]
impl Saver for LocalSaver {
    /// saves to local dir
    async fn save(&self, log: SessionLog, file_name: &str) -> bool {
        debug!(
            "Save to dir: {} file: {} log: {:?} t",
            self.dir, file_name, log
        );
        let dir = Path::new(self.dir.as_str());
        if !dir.exists() {
            if let Err(e) = fs::create_dir_all(dir) {
                error!("Cannot create dir: {e}");
                return false;
            }
        }
        let mut full_path = String::new();
        full_path.push_str(&self.dir);
        full_path.push('/');
        full_path.push_str(file_name);

        match serde_json::to_string_pretty(&log) {
            Ok(data) => {
                if let Err(e) = std::fs::write(&full_path, data) {
                    error!("Cannot write {} {}", full_path, e);
                } else {
                    return true;
                }
            }
            Err(e) => {
                error!("Cannot convert log to json: {:?}", e);
            }
        }

        false
    }
    /// nothing for now in this init method
    async fn init(&self, _cfg: &Cfg) {}
    /// name getter
    fn get_name(&self) -> String {
        "local-fs".to_string()
    }
}
