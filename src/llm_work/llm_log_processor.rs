use crate::llm_work::reducter::ReDucter;
use crate::storage::saver::Saver;
use std::collections::HashSet;
use std::sync::Arc;

/// Processes LLM logs with redaction capabilities
pub struct LlmLogProcessor {
    /// LLM API caller implementation
    pub caller: Arc<dyn ReDucter + Send + Sync>,

    /// System prompt for LLM interactions
    pub system_prompt: String,

    /// Model name/identifier for LLM calls
    pub model: String,

    /// Storage implementation for saving processed logs
    pub saver: Arc<dyn Saver + Send + Sync>,

    /// Set of valid redaction tags (None means all are valid)
    pub valid_redactions: Option<HashSet<String>>,
}
