use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::pii_text::pii_text;
use std::str::from_utf8;
use tracing::{debug, error};

impl LlmLogProcessor {
    pub async fn process(&self, payload: Vec<u8>) -> bool {
        if let Ok(text) = from_utf8(&payload) {
            debug!("Payload: {}", text);
        } else {
            debug!("Payload (not valid UTF-8): {:?}", payload);
        }
        if let Some(log) = Self::parse(payload) {
            let redaction_text = pii_text(log);
            debug!("history: {:?}", redaction_text);
            let prompt = self.system_prompt.clone();
            let result = self
                .caller
                .call(
                    self.model.as_str(), //
                    prompt.as_str(),
                    &redaction_text, /* &str */
                )
                .await;

            debug!("Result: {:?}", result);
            match result {
                Some(v) => {
                    let choices = &v["choices"];
                    debug!("Choices: {}", choices);
                }
                None => {
                    error!("Wrong response {:?}", result);
                }
            }
        }
        true
    }
}
