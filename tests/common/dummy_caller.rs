use async_trait::async_trait;
use ductaper::init_logging::init_tracing;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::llm_work::reducter::ReDucter;
use ductaper::session_log_models::SessionLog;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tracing::{debug, info};




pub struct DummyCaller {}
#[async_trait]
impl ReDucter for DummyCaller {
    async fn call(&self, _model: &str, _prompt: &str, _message: &str) -> Option<Value> {
        debug!("call");
        Some(json!({}))
    }
}
