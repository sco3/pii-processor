use std::sync::Arc;
use crate::reducter::ReDucter;

pub struct LlmLogProcessor {
    pub prompt_location: String,
    pub caller: Arc<dyn ReDucter + Send + Sync>,
    pub system_prompt: Option<String>,
}
