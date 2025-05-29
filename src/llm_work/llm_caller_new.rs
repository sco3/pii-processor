use crate::llm_work::llm_caller::LLmCaller;
use moka::future::Cache;
use std::time::Duration;

impl LLmCaller {
    #[must_use]
    /// Creates new `LLmCaller` instance
    pub fn new(
        endpoint: &str,
        model: &str,
        token: Option<&String>,
        cache_flag: bool,
        cache_sleep_millis: u64,
    ) -> Self {
        let bearer = Self::add_bearer(token);
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(3600))
            .max_capacity(1000)
            .build();

        LLmCaller {
            endpoint: endpoint.into(),
            model: model.into(),
            bearer,
            client: reqwest::Client::new(),
            cache_flag,
            cache,
            cache_sleep_millis,
        }
    }
}
