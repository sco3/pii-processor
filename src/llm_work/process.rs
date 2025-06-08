use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::session_log_models::SessionLogEntry::ChatMessage;
use tracing::debug;

impl LlmLogProcessor {
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
