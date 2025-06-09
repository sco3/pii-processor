use bytes::Bytes;
use ductaper::init_logging::init_tracing;
use ductaper::llm_caller::LLmCaller;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use std::fs::{read, read_to_string};
use std::sync::Arc;
use tracing::debug;
const TOKEN: &str = "sk-1234";
const URL: &str = "http://0.0.0.0:4000/chat/completions";

#[tokio::main]
async fn main() {
    init_tracing();

    let models = vec![/*"nova",*/ "haiku"];
    debug!("Models: {:?}", models);
    let system_prompt = read_to_string("data/system_prompt.txt") //
        .unwrap();
    let preview: String = system_prompt.chars().take(42).collect();
    debug!("System prompt: {}...", preview);

    let session_log = read("tests/data/worker-pool-test.json").unwrap();

    let buf = Bytes::copy_from_slice(&session_log);

    let log_preview = &buf[..buf.len().min(42)];
    let log_preview_str = String::from_utf8_lossy(log_preview);
    debug!("Session log {:?}", log_preview_str);

    let caller = Arc::new(LLmCaller::new(
        URL, //
        "haiku",
        Some(TOKEN.to_string()),
    ));

    for model in models {
        let processor = LlmLogProcessor::new(
            caller.clone(), //
            system_prompt.clone(),
            model.to_string(),
        );

        let bytes = Bytes::copy_from_slice(&session_log);

        processor.process(bytes).await;
    }
}
