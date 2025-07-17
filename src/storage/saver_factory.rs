use crate::config::env_vars::Cfg;
use crate::probe::toggle::Toggle;
use crate::storage::get_bucket::get_bucket;
use crate::storage::local_saver::LocalSaver;
use crate::storage::s3_saver::S3Saver;
use crate::storage::saver::Saver;
use std::sync::Arc;

/// creates saver based on configured env value
/// # Panics
///
/// Panics if `cfg.aggregator_sessions_log_url` starts with "s3://"
/// but is not a valid S3 URL from which a bucket name can be successfully
/// extracted by the `get_bucket` function. This indicates a critical
/// misconfiguration, and the program will exit.
///
pub async fn get_saver(cfg: &Cfg, storage_toggle: Toggle) -> Arc<dyn Saver + Send + Sync> {
    if cfg.aggregator_sessions_log_url.starts_with("s3://") {
        let bucket = get_bucket(cfg.aggregator_sessions_log_url.as_str()).unwrap();

        let saver = S3Saver::new(bucket.as_str(), cfg, storage_toggle).await;
        Arc::new(saver)
    } else {
        let local_saver = Arc::new(
            LocalSaver::new(&cfg.aggregator_sessions_log_url.clone()), //
        );
        storage_toggle.set_ready(true);
        local_saver
    }
}
