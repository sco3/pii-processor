use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::reducter::ReDucter;
use std::sync::Arc;

impl LlmLogProcessor {
    pub fn new(
        caller: Arc<dyn ReDucter + Send + Sync>, //
        system_prompt: String,
    ) -> Self {
        LlmLogProcessor {
            caller,
            system_prompt,
        }
    }
}
