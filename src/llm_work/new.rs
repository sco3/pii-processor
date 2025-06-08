use crate::llm_caller_trait::ReDucter;
use crate::llm_work::LlmLogProcessor;
use std::sync::Arc;
use tokio::sync::Mutex;

impl LlmLogProcessor {
    pub fn new(prompt_location: String, caller: Arc<Mutex<dyn ReDucter + Send + Sync>>) -> Self {
        LlmLogProcessor {
            prompt_location,
            caller,
            system_prompt: None,
        }
    }
}
