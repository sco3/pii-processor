use super::LlmLogProcessor;
use crate::llm_caller_trait::ReDucter;

use crate::session_log_models::SessionLogEntry::ChatMessage;
use std::fs::read_to_string;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::debug;

impl LlmLogProcessor {
    pub fn new(prompt_location: String, caller: Arc<Mutex<dyn ReDucter + Send + Sync>>) -> Self {
        LlmLogProcessor {
            prompt_location,
            caller,
            system_prompt: None,
        }
    }

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

    pub async fn process(&mut self, payload: &[u8]) -> bool {
        if let Some(log) = Self::parse(&payload) {
            let mut chat_history = Vec::new();
            for entry in log {
                if let ChatMessage(msg) = entry {
                    chat_history.push(msg);
                }
            }
            if let Ok(pii_message) = serde_json::to_string(&chat_history) {
                debug!("history: {:?}", pii_message);
                let prompt = self.prompt();
                let result = self
                    .caller
                    .lock()
                    .await
                    .call(prompt.as_str(), &pii_message)
                    .await;

                debug!("Result: {:?}", result);
            }
        }
        true
    }
}
