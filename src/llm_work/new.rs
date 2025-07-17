use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::llm_work::masks::get_valid_redactions;
use crate::llm_work::reducter::ReDucter;
use crate::storage::saver::Saver;
use std::sync::Arc;
use tracing::info;

impl LlmLogProcessor {
    /// llm log processor constructor
    pub fn new(
        caller: Arc<dyn ReDucter + Send + Sync>, //
        system_prompt: String,
        model: &str,
        saver: Arc<dyn Saver + Send + Sync>,
    ) -> Self {
        let valid_redactions = get_valid_redactions(
            system_prompt.as_str(), //
        );
        info!(
            "Redaction masks were parsed from system prompt. \
            Found redaction masks: {:?}.",
            valid_redactions,
        );
        LlmLogProcessor {
            caller,
            system_prompt,
            model: model.to_string(),
            saver,
            valid_redactions,
        }
    }
}
