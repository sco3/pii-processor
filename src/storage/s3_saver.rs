use crate::session_log_models::SessionLogType;
use crate::storage::s3helper::S3Helper;
use crate::storage::saver::Saver;
use async_trait::async_trait;
use tracing::error;
struct S3Saver {
    s3helper: S3Helper,
    bucket: String,
}

#[async_trait]
impl Saver for S3Saver {
    async fn save(&self, log: SessionLogType, file_name: &str) -> bool {
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
