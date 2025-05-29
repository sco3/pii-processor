use crate::worker_pool::serve::Stat;
use async_trait::async_trait;
use serde_json::Value;

/// trait (interface for calling llm and tests)
#[async_trait]
pub trait ReDucter: Send + Sync {
    /// method to call llm
    async fn call(
        &self, //
        model: &str,
        prompt: &str,
        message: &str,
        stat: &mut Stat,
    ) -> Option<Value>;
}
