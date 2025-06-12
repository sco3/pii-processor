use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::llm_work::reducter::ReDucter;
use crate::storage::saver::Saver;
use std::sync::Arc;

impl LlmLogProcessor {
    pub fn new(
        caller: Arc<dyn ReDucter + Send + Sync>, //
        system_prompt: String,
        model: String,
        saver: Arc<dyn Saver + Send + Sync>,
    ) -> Self {
        LlmLogProcessor {
            caller,
            system_prompt,
            model,
            saver,
        }
    }
}
