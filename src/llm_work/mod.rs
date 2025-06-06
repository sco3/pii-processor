mod impls;
mod parse;

use crate::llm_caller_trait::ReDucter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LlmLogProcessor {
    pub prompt_location: String,
    pub caller: Arc<Mutex<dyn ReDucter + Send + Sync>>,
    pub system_prompt: Option<String>,
}
