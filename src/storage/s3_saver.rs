use crate::session_log_models::SessionLog;
use crate::storage::s3helper::S3Helper;
use crate::storage::saver::Saver;
use async_trait::async_trait;
use tracing::{debug, error};
pub struct S3Saver {
    pub s3helper: S3Helper,
    pub bucket: String,
}

#[async_trait]
impl Saver for S3Saver {
    async fn save(&self, log: SessionLog, file_name: &str) -> bool {
        debug!("Save to key: {} log: {:?} t", file_name, log);
        match serde_json::to_string_pretty(&log) {
            Ok(data) => {
                self.s3helper
                    .put_object(
                        self.bucket.clone(),
                        file_name.to_string(),
                        data.into_bytes(),
                    )
                    .await;
                return true;
            }
            Err(e) => {
                error!("Cannot convert log to json: {:?}", e);
            }
        }
        false
    }
}
