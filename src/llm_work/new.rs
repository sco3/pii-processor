use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::llm_work::masks::get_valid_redactions;
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
        let valid_redactions = get_valid_redactions(
            system_prompt.as_str(), //
        );
        LlmLogProcessor {
            caller,
            system_prompt,
            model,
            saver,
            valid_redactions,
        }
    }
}
