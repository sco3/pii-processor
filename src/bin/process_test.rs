use async_trait::async_trait;
use redact::data::session_log_models::SessionLog;
use redact::llm_work::llm_caller::LLmCaller;
use redact::llm_work::llm_log_processor::LlmLogProcessor;
use redact::storage::saver::Saver;
use redact::util::logging::init_tracing;
use redact::worker_pool::serve::Stat;
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
    fn get_name(&self) -> String {
        "DummySaver".to_string()
    }
    // / init dummy saver - nothing to do
    //async fn init(&mut self, _cfg: &Cfg) -> bool {
    //true
    //}
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
        Some(&TOKEN.to_string()),
        false,
        0,
    ));

    for model in models {
        let processor = LlmLogProcessor::new(
            caller.clone(), //
            system_prompt.clone(),
            model,
            Arc::new(DummySaver {}),
        );
        let mut stat = Stat::default();
        processor
            .process(
                session_log.clone(), //
                "asdf.json",
                &mut stat,
            )
            .await;
    }
}
