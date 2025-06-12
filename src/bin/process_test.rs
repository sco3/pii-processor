use async_trait::async_trait;
use ductaper::init_logging::init_tracing;
use ductaper::llm_work::llm_caller::LLmCaller;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::session_log_models::SessionLog;
use ductaper::storage::saver::Saver;
use std::fs::{read, read_to_string};
use std::sync::Arc;
use tracing::debug;

const TOKEN: &str = "sk-1234";
const URL: &str = "http://0.0.0.0:4000/chat/completions";

pub struct DummySaver {}
#[async_trait]
impl Saver for DummySaver {
    async fn save(&self, _log: SessionLog, _file_name: &str) -> bool {
        true
    }
}

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

    debug!("Session log {:?}", session_log);

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
            Arc::new(DummySaver {}),
        );

        processor.process(session_log.clone(), "asdf.json").await;
    }
}
