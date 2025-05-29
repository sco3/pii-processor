use crate::llm_work::llm_caller::LLmCaller;
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::info;

impl LLmCaller {
    /// Checks cache and falls back to direct send if miss
    pub(crate) async fn check_cache(
        &self,
        start: Instant,
        body: &Value,
        message: &str,
    ) -> Option<Value> {
        if let Some(v) = self.cache.get(body).await {
            // sleep to emulate work
            let sleep_duration = Duration::from_millis(self.cache_sleep_millis);
            sleep(sleep_duration).await;
            info!(
                "Cache hit: {} us cache size: {} sleep: {} us",
                start.elapsed().as_micros(),
                self.cache.entry_count(),
                sleep_duration.as_micros()
            );
            Some(v)
        } else {
            let value = self.direct_send(start, body, message).await;
            if let Some(ref ref_value) = value {
                self.cache.insert(body.clone(), ref_value.clone()).await;
            }
            value
        }
    }
}
