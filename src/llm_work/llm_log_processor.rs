use crate::llm_work::reducter::ReDucter;
use crate::storage::saver::Saver;
use std::collections::HashSet;
use std::sync::Arc;

pub struct LlmLogProcessor {
    pub caller: Arc<dyn ReDucter + Send + Sync>,
    pub system_prompt: String,
    pub model: String,
    pub saver: Arc<dyn Saver + Send + Sync>,
    pub valid_redactions: Option<HashSet<String>>,
}

