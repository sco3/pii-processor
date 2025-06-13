use crate::data::session_log_models::SessionLog;
use crate::storage::saver::Saver;
use async_trait::async_trait;
use tracing::{debug, error};

pub struct LocalSaver {
    pub dir: String,
}
impl LocalSaver {
    pub fn new(dir: String) -> Self {
        LocalSaver { dir }
    }
}

#[async_trait]
impl Saver for LocalSaver {
    async fn save(&self, log: SessionLog, file_name: &str) -> bool {
        debug!(
            "Save to dir: {} file: {} log: {:?} t",
            self.dir, file_name, log
        );
        let mut full_path = String::new();
        full_path.push_str(&self.dir);
        full_path.push_str("/");
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
}
