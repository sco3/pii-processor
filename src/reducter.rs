use async_trait::async_trait;
use serde_json::Value;

#[async_trait]
pub trait ReDucter: Send + Sync {
    async fn call(&self, prompt: &str, message: &str) -> Option<Value>;
}
