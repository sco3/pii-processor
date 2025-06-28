use crate::config::env_vars::Cfg;
use crate::data::session_log_models::SessionLog;
use crate::probe::toggle::Toggle;
use std::error::Error;

use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3helper::S3Helper;
use crate::storage::saver::Saver;
use crate::util::exit_codes::ExitCode;
use async_trait::async_trait;
use std::process::exit;
use std::sync::Arc;
use tracing::{debug, error, info};

use tokio::sync::OnceCell;

/// structure to save to s3
pub struct S3Saver {
    /// utility class to perform s3 operations
    pub s3helper: OnceCell<S3Helper>,
    /// bucket name
    pub bucket: Arc<str>,
    /// http probe toggler - sets ready/not ready based on s3 operation
    pub toggle: Toggle,
}

impl S3Saver {
    /// init s3 context
    /// # Errors
    /// returned when s3 context creation failed
    pub async fn get_s3_ctx(bucket: &str, cfg: &Cfg) -> Result<S3Ctx, Box<dyn Error>> {
        let region = cfg.aws_region_s3.clone();
        info!("S3 init: {bucket} {region}");

        let access_key: Option<String> = cfg //
            .aws_access_key_id
            .clone()
            .map(|v| v.get_string());

        let secret_key: Option<String> = cfg //
            .aws_secret_access_key
            .clone()
            .map(|v| v.get_string());

        let access_token: Option<String> = cfg //
            .aws_access_token
            .clone()
            .map(|v| v.get_string());

        match S3Ctx::new(
            bucket.to_string(),
            region,
            access_key,
            secret_key,
            access_token,
            cfg.aws_s3_endpoint.clone(),
        )
        .await
        {
            Ok(ctx) => Ok(ctx),
            Err(e) => Err(e),
        }
    }

    // / Constructor
    // /
    // pub async fn new(bucket: &str, cfg: &Cfg, toggle: Toggle) -> Self {
    //     match S3Saver::get_s3_ctx(bucket, cfg).await {
    //         Ok(s3ctx) => {
    //             toggle.set_ready(true);
    //             S3Saver {
    //                 s3helper: Some(S3Helper::new(s3ctx)),
    //                 bucket: Arc::from(bucket),
    //                 toggle,
    //             }
    //         }
    //         Err(e) => {
    //             error!("S3 problem: {}", e);
    //             exit(ExitCode::S3Error.code());
    //         }
    //     }
    // }

    /// lazy constructor
    pub fn new_lazy(bucket: &str, toggle: Toggle) -> Self {
        info!("S3Saver created in lazy mode for bucket: {}", bucket);
        S3Saver {
            s3helper: OnceCell::new(), // Starts uninitialized
            bucket: Arc::from(bucket), // Store the bucket name
            toggle,                    // Store the toggle
        }
    }
}

#[async_trait]
impl Saver for S3Saver {
    /// save to s3
    async fn save(&self, log: SessionLog, file_name: &str) -> bool {
        debug!("Save to key: {} log: {:?} t", file_name, log);
        if let Some(helper) = self.s3helper.get() {
            match serde_json::to_string_pretty(&log) {
                Ok(data) => {
                    let out = helper
                        .put_object(&self.bucket, file_name.to_string(), data.into_bytes())
                        .await;
                    self.toggle.set_ready(out);
                    return out;
                }
                Err(e) => {
                    error!("Cannot convert log to json: {:?}", e);
                }
            }
        }
        false
    }
    /// creates connectin for s3
    async fn init(&self, cfg: &Cfg) {
        if self.s3helper.get().is_some() {
            return;
        }

        match S3Saver::get_s3_ctx(&self.bucket, cfg).await {
            Ok(ctx) => {
                let _ = self.s3helper.set(S3Helper::new(ctx));
                self.toggle.set_ready(true);
            }
            Err(e) => {
                error!("S3 init failed: {e}");
                self.toggle.set_ready(false);
                exit(ExitCode::S3Error.code());
            }
        }
    }

    /// returns name
    fn get_name(&self) -> String {
        "s3".to_string()
    }
}
