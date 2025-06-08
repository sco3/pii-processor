use async_trait::async_trait;
use ductaper::llm_caller::LLmCaller;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::reducter::ReDucter;
use serde_json::Value;
use std::fs;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

struct _DummyLlmCaller;

#[async_trait]
impl ReDucter for _DummyLlmCaller {
    async fn call(&self, prompt: &str, message: &str) -> Option<Value> {
        debug!("Dummy LLM call with prompt: {} {}", prompt, message);
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

    let mut processor = LlmLogProcessor {
        prompt_location: "//tmp".to_string(),
        caller,
        system_prompt: None,
    };

    // Load test file from relative path
    let path = "tests/data/example_new_fields.json";
    let file_content = fs::read(path) //
        .expect("Failed to read example_new_fields.log");

    // Process the payload
    let result = processor.process(&file_content).await;
    info!("Processing result: {}", result);
}
