use crate::llm_work::check_or_swap::check_or_swap;
use crate::llm_work::llm_log_processor::LlmLogProcessor;
use serde_json;
use serde_json::Value;
use std::collections::HashMap;
use tracing::debug;

impl LlmLogProcessor {
    /// parse llm output to find original string and replacements
    #[must_use]
    pub fn parse_redactions(&self, content: &str) -> Option<HashMap<String, String>> {
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
                    if replace_with.starts_with('[') && replace_with.ends_with(']') {
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
    /// redactions getter
    pub fn redactions(&self, value: &Value) -> HashMap<String, String> {
        if let Some(content) = Self::extract_content(value) {
            debug!("Content: {:?}", content);
            if let Some(redactions) = self.parse_redactions(content) {
                debug!("Redactions: {:?}", redactions);
                return redactions;
            }
        }
        HashMap::new()
    }
}
