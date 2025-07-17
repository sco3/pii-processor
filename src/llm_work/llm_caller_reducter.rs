use crate::llm_work::llm_caller::LLmCaller;
use crate::llm_work::reducter::ReDucter;
use crate::worker_pool::serve::Stat;
use async_trait::async_trait;
use serde_json::Value;
use std::time::Instant;

#[async_trait]
impl ReDucter for LLmCaller {
    /// Makes LLM API call with optional caching
    async fn call(
        &self,
        model: &str,
        prompt: &str,
        message: &str,
        stat: &mut Stat,
    ) -> Option<Value> {
        let start_call = Instant::now();
        let body = Self::build_body(model, prompt, message);
        self.debug_body(&body);
        if self.cache_flag {
            return self.check_cache(start_call, &body, message, stat).await;
        }
        self.direct_send(start_call, &body, message, stat).await
    }
}
