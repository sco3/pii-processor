use crate::llm_work::reducter::ReDucter;
use std::sync::Arc;

pub struct LlmLogProcessor {
    pub caller: Arc<dyn ReDucter + Send + Sync>,
    pub system_prompt: String,
    pub model: String,
}
