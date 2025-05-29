use crate::config::env_vars::Cfg;
use crate::data::session_log_models::SessionLog;
use crate::probe::toggle::Toggle;
use crate::storage::get_bucket::get_bucket;
use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3helper::S3Helper;
use crate::storage::saver::Saver;
use crate::util::exit_codes::ExitCode;
use async_trait::async_trait;
use std::process::exit;
use tracing::{debug, error};

/// structure to save to s3
pub struct S3Saver {
    /// utility class to perform s3 operations
    pub s3helper: Option<S3Helper>,
    /// bucket name
    pub bucket: String,
    /// http probe toggler - sets ready/not ready based on s3 operation
    pub toggle: Toggle,
}

#[async_trait]
impl Saver for S3Saver {
    /// save to s3
    async fn save(&self, log: SessionLog, file_name: &str) -> bool {
        debug!("Save to key: {} log: {:?} t", file_name, log);
        if let Some(helper) = &self.s3helper {
            match serde_json::to_string_pretty(&log) {
                Ok(data) => {
                    let out = helper
                        .put_object(
                            self.bucket.clone(),
                            file_name.to_string(),
                            data.into_bytes(),
                        )
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

    async fn init(&mut self, cfg: &Cfg) -> bool {
        let bucket = get_bucket(cfg.aggregator_sessions_log_url.as_str()).unwrap();

        let access_key: Option<String> = cfg.aws_access_key_id.clone().map(|v| v.get_string());
        let secret_key: Option<String> = cfg.aws_secret_access_key.clone().map(|v| v.get_string());
        let access_token: Option<String> = cfg.aws_access_token.clone().map(|v| v.get_string());

        let s3ctx = match S3Ctx::new(
            bucket.clone(),
            cfg.aws_region_s3.clone(),
            access_key,
            secret_key,
            access_token,
            cfg.aws_s3_endpoint.clone(),
        )
        .await
        {
            Ok(ctx) => {
                self.toggle.set_ready(true);
                ctx
            }
            Err(e) => {
                error!("S3 problem: {}", e);
                exit(ExitCode::S3Error.code());
            }
        };

        self.s3helper = Some(S3Helper::new(s3ctx));
        true
    }
    /// returns name
    fn get_name(&self) -> String {
        "s3".to_string()
    }
}
