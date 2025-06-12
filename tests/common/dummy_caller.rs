use async_trait::async_trait;
use ductaper::llm_work::reducter::ReDucter;
use serde_json::{json, Value};
use tracing::debug;




pub struct DummyCaller {}
#[async_trait]
impl ReDucter for DummyCaller {
    async fn call(&self, _model: &str, _prompt: &str, _message: &str) -> Option<Value> {
        debug!("call");
        Some(json!({}))
    }
}
