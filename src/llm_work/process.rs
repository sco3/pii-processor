use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::pii_text::pii_text;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, error, Level};

impl LlmLogProcessor {
    pub async fn process(&self, payload: Vec<u8>, file_name: &str) {
        Self::debug("Payload", &payload);

        let Some(mut log) = Self::parse(payload.clone()) else {
            Self::error("Parse error", &payload);
            return;
        };

        let redaction_text = pii_text(&log);
        debug!("history: {}", redaction_text);

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
            self.update_log(&mut log, &redacts);
        }
        debug!("Save result to {}", file_name);
        self.saver.save(log, file_name).await;
    }

    fn debug(label: &str, payload: &Vec<u8>) {
        if tracing::enabled!(Level::DEBUG) {
            match str::from_utf8(payload) {
                Ok(text) => debug!("{} (non-UTF-8 bytes): {}", label, text),
                Err(_) => debug!("{} (non-UTF-8 bytes): {:?}", label, payload),
            }
        }
    }
    fn error(label: &str, payload: &Vec<u8>) {
        if tracing::enabled!(Level::DEBUG) {
            match str::from_utf8(payload) {
                Ok(text) => error!("{} (non-UTF-8 bytes): {}", label, text),
                Err(_) => error!("{} (non-UTF-8 bytes): {:?}", label, payload),
            }
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

    ///
    fn parse_redactions(&self, content: &str) -> Option<HashMap<String, String>> {
        let parsed: Value = serde_json::from_str(content).ok()?;
        let redactions = parsed.get("redactions")?.as_object()?;

        let mut result = HashMap::new();
        for (key, value) in redactions {
            if let Some(replace_with) = value.as_str() {
                // sometimes LLM creates redactions not from the system prompt
                // they should be filtered out (non valid are not inserted).
                if let Some(vr) = &self.valid_redactions {
                    // valid redactions found and they include replacement
                    if vr.contains(replace_with) {
                        result.insert(key.clone(), replace_with.to_string());
                    }
                } else {
                    // valid redaction not found this is most unlikely event
                    result.insert(key.clone(), replace_with.to_string());
                }
            }
        }

        Some(result)
    }
    fn redactions(&self, response: Option<Value>) -> Option<HashMap<String, String>> {
        let value = response?;
        let content = Self::extract_content(&value)?;
        debug!("Content: {:?}", content);

        let redactions = self.parse_redactions(content)?;
        debug!("Redactions: {:?}", redactions);
        Some(redactions)
    }
}
