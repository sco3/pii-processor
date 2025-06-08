use crate::llm_caller_trait::ReDucter;
use crate::llm_work::LlmLogProcessor;
use crate::session_log_models::SessionLogEntry::ChatMessage;
use std::fs::read_to_string;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

impl LlmLogProcessor {
    pub fn prompt(&mut self) -> String {
        if let Some(system_prompt) = self.system_prompt.clone() {
            self.system_prompt = Some(system_prompt);
            return self.system_prompt.clone().unwrap();
        }
        read_to_string(&self.prompt_location) //
            .unwrap_or_else(|e| {
                debug!("Failed to read system prompt: {}", e);
                String::new()
            })
    }
}
