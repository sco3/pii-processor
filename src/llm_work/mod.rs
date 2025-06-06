mod impls;
mod parse;

use crate::env_vars::Cfg;
use crate::llm_caller_trait::ReDucter;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct LlmLogProcessor {
    cfg: Cfg,
    pub caller: Arc<Mutex<dyn ReDucter + Send + Sync>>,
    system_prompt: Option<String>,
}
