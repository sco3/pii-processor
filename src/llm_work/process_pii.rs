use crate::data::ai_tags::Ai;
use crate::llm_work::check_or_swap::check_or_swap;
use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::pii_text::pii_text;
use crate::llm_work::process_result::ProcessResult;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use tracing::{debug, error, Level};

impl LlmLogProcessor {
    pub async fn process(&self, payload: Vec<u8>, file_name: &str) -> ProcessResult {
        Self::debug("Payload", &payload);

        let Some(mut log) = Self::parse(payload.clone()) else {
            Self::error("Parse error", &payload);
            return ProcessResult::ParseError;
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

        if let Some(r) = response {
            //replace redacted strings
            let redacts = self.redactions(r);
            if !redacts.is_empty() {
                self.update_log(&mut log, &redacts);
            }
            debug!("Save result to {}", file_name);
            self.saver.save(log, file_name).await;
            return ProcessResult::Ok;
        } else {
            error!("LLM failure")
        }
        ProcessResult::Error
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

    pub fn extract_content_old(value: &Value) -> Option<&str> {
        match value
            .get(Ai::CHOICES)
            .and_then(|v| v.get(0))
            .and_then(|v| v.get(Ai::MESSAGE))
            .and_then(|v| v.get(Ai::CONTENT))
            .and_then(|v| v.as_str())
        {
            Some(s) => Some(s),
            None => {
                error!(
                    "Missing expected JSON fields: .{}[0].{}.{} in response: {:?}",
                    Ai::CHOICES,
                    Ai::MESSAGE,
                    Ai::CONTENT,
                    value
                );
                None
            }
        }
    }
    pub fn extract_content(value: &Value) -> Option<&str> {
        if let Some(s) = value["choices"][0]["message"]["content"].as_str() {
            return Some(s);
        } else {
            error!(
                "Missing expected JSON fields: {} in response: {}",
                ".choices[0].message.content", value
            );
        }
        None
    }

    /// parse llm response redactions
    fn parse_redactions(&self, content: &str) -> Option<HashMap<String, String>> {
        let parsed: Value = serde_json::from_str(content).ok()?;
        let redactions = parsed.get("redactions")?.as_object()?;

        let mut result: HashMap<String, String> = HashMap::new();
        for (maybe_key, maybe_json_value) in redactions {
            if let Some(maybe_value) = maybe_json_value.as_str() {
                let (key, replace_with) = check_or_swap(maybe_key, maybe_value);

                // sometimes LLM creates redactions not from the system prompt
                // they should be filtered out (non valid are not inserted).
                if let Some(vr) = &self.valid_redactions {
                    // valid redactions found and they include replacement
                    if replace_with.starts_with("[") && replace_with.ends_with("]") {
                        // full redactions like [person]
                        if vr.contains(replace_with) {
                            result.insert(key.to_string(), replace_with.to_string());
                        }
                    } else {
                        // partial redactions like 1234*** - no need to validate
                        result.insert(key.to_string(), replace_with.to_string());
                    }
                } else {
                    // valid redaction not found this is most unlikely event
                    result.insert(key.to_string(), replace_with.to_string());
                }
            }
        }

        Some(result)
    }
    fn redactions(&self, value: Value) -> HashMap<String, String> {
        if let Some(content) = Self::extract_content(&value) {
            debug!("Content: {:?}", content);
            if let Some(redactions) = self.parse_redactions(content) {
                debug!("Redactions: {:?}", redactions);
                return redactions;
            }
        }
        HashMap::new()
    }
}
