use async_trait::async_trait;
use ductaper::llm_caller::LLmCaller;
use ductaper::llm_caller_trait::ReDucter;
use ductaper::llm_log_processor::LlmLogProcessor;
use serde_json::Value;
use std::fs;
use std::sync::{Arc, Mutex};
use tracing::info;

struct DummyLlmCaller;

#[async_trait]
impl ReDucter for DummyLlmCaller {
    async fn call(&self, prompt: &str, message: &str) -> Option<Value> {
        None
    }
}

#[tokio::test]
async fn test_llm_log_processor() {
    //let caller = Arc::new(Mutex::new(DummyLlmCaller {}));
    let raw_caller = LLmCaller::new(
        "http://0.0.0.0:4000/chat/completions",
        "nova",
        Some("sk-1234".to_string()),
    );
    let caller = Arc::new(Mutex::new(raw_caller));

    let processor = LlmLogProcessor { caller };

    // Load test file from relative path
    let path = "tests/data/example_new_fields.json";
    let file_content = fs::read(path) //
        .expect("Failed to read example_new_fields.log");

    // Process the payload
    let result = processor.process(&file_content);
    info!("Processing result: {}", result);
}
