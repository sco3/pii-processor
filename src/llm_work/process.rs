use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::pii_text::pii_text;
use crate::session_log_models::SessionLog;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, error};

impl LlmLogProcessor {
    fn update_log(&self, _log: SessionLog, _redacts: HashMap<String, String>) {
        
    }

    pub async fn process(&self, payload: Vec<u8>) {
        self.log_payload(&payload);

        let Some(log) = Self::parse(payload.clone()) else {
            error!("Session log format error: {:?}", payload);
            return;
        };

        let redaction_text = pii_text(&log);
        debug!("history: {:?}", redaction_text);

        let response = self
            .caller
            .call(
                self.model.as_str(),
                self.system_prompt.as_str(),
                &redaction_text,
            )
            .await;
        //replace redacted strings
        let redacts = self.redactions(response).unwrap_or_default();
        if !redacts.is_empty() {
            self.update_log(log, redacts);
        }
    }

    fn log_payload(&self, payload: &[u8]) {
        match std::str::from_utf8(payload) {
            Ok(text) => debug!("Payload: {}", text),
            Err(_) => debug!("Payload (not valid UTF-8): {:?}", payload),
        }
    }

    fn extract_content(value: &Value) -> Option<&str> {
        match value
            .get("choices")
            .and_then(|v| v.get(0))
            .and_then(|v| v.get("message"))
            .and_then(|v| v.get("content"))
            .and_then(|v| v.as_str())
        {
            Some(s) => Some(s),
            None => {
                error!("Missing expected JSON fields in response: {:?}", value);
                None
            }
        }
    }

    fn parse_redactions(content: &str) -> Option<HashMap<String, String>> {
        match serde_json::from_str::<Value>(content) {
            Ok(v) => v
                .get("redactions")
                .and_then(|val| val.as_object())
                .map(|obj| {
                    obj.iter()
                        .filter_map(|(k, v)| v.as_str().map(|s| (k.clone(), s.to_string())))
                        .collect()
                }),
            Err(e) => {
                error!("Invalid json in content: {}", e);
                None
            }
        }
    }

    fn redactions(&self, response: Option<Value>) -> Option<HashMap<String, String>> {
        let value = response?;
        let content = Self::extract_content(&value)?;
        debug!("Content: {:?}", content);

        let redactions = Self::parse_redactions(content)?;
        debug!("Redactions: {:?}", redactions);
        Some(redactions)
    }
}
