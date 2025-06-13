use crate::config::env_vars::Cfg;
use crate::probe::toggle::Toggle;
use crate::storage::get_bucket::get_bucket;
use crate::storage::local_saver::LocalSaver;
use crate::storage::s3_saver::S3Saver;
use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3helper::S3Helper;
use crate::storage::saver::Saver;
use crate::util::exit_codes::ExitCode;
use std::process::exit;
use std::sync::Arc;
use tracing::error;

pub async fn get_saver(cfg: &Cfg, storage_toggle: Toggle) -> Arc<dyn Saver + Send + Sync> {
    if cfg.aggregator_sessions_log_url.starts_with("s3://") {
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
            None,
        )
        .await
        {
            Ok(ctx) => {
                storage_toggle.set_ready(true);
                ctx
            }
            Err(e) => {
                error!("S3 problem: {}", e);
                exit(ExitCode::S3Error.code());
            }
        };

        let s3helper = S3Helper::new(s3ctx);

        (Arc::new(S3Saver {
            s3helper,
            bucket,
            toggle: storage_toggle,
        })) as _
    } else {
        let local_saver = Arc::new(
            LocalSaver::new(cfg.aggregator_sessions_log_url.clone()), //
        );
        storage_toggle.set_ready(true);
        local_saver
    }
}
