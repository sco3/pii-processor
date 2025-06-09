use crate::reducter::ReDucter;
use std::sync::Arc;

pub struct LlmLogProcessor {
    pub caller: Arc<dyn ReDucter + Send + Sync>,
    pub system_prompt: String,
    pub model: String,
}
