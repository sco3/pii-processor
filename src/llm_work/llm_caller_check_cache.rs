use crate::llm_work::llm_caller::LLmCaller;
use crate::worker_pool::serve::Stat;
use serde_json::Value;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::debug;

impl LLmCaller {
    /// Checks cache and falls back to direct send if miss
    pub(crate) async fn check_cache(
        &self,
        start_processing: Instant,
        body: &Value,
        message: &str,
        stat: &mut Stat,
    ) -> Option<Value> {
        let get_start = Instant::now();
        if let Some(v) = self.cache.get(message) {
            let get_took = get_start.elapsed().as_micros();
            // sleep to emulate work
            let sleep_duration = Duration::from_millis(self.cache_sleep_millis);
            sleep(sleep_duration).await;
            stat.cache_hit_micros = start_processing.elapsed().as_micros();
            stat.cache_get_micros = get_took;
            stat.cache_len = self.cache.entry_count();
            debug!(
                "Cache hit: {} us cache get: {get_took} us cache size: {} sleep: {} us",
                start_processing.elapsed().as_micros(),
                self.cache.entry_count(),
                sleep_duration.as_micros()
            );
            Some(v)
        } else {
            let value = self
                .direct_send(start_processing, body, message, stat)
                .await;
            if let Some(ref ref_value) = value {
                self.cache.insert(Arc::from(message), ref_value.clone());
            }
            value
        }
    }
}
