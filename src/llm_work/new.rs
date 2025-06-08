use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::reducter::ReDucter;
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
