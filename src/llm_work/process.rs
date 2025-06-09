use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::texter::extract_text;
use bytes::Bytes;
use tracing::debug;

impl LlmLogProcessor {
    pub async fn process(&self, payload: Bytes) -> bool {
        if let Some(log) = Self::parse(payload) {
            let redaction_text = extract_text(log);
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
        }
        true
    }
}
