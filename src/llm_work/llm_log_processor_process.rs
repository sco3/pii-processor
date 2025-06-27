use crate::llm_work::get_text_from_session_log::get_text_from_session_log;
use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::worker_pool::serve::Stat;
use serde_json;
use serde_json::Value;
use tokio::time::Instant;
use tracing::{Level, debug, error};

impl LlmLogProcessor {
    /// process message with session log
    pub async fn process(
        &self, //
        payload: Vec<u8>,
        file_name: &str,
        stat: &mut Stat,
    ) -> ProcessResult {
        Self::debug("Payload", &payload);

        let start_parse = Instant::now();
        let Some(mut log) = Self::parse(&payload) else {
            Self::error("Parse error", &payload);
            return ProcessResult::ParseError;
        };
        stat.parse_us = start_parse.elapsed().as_micros();

        let start_extract = Instant::now();
        let redaction_text = get_text_from_session_log(&log);
        stat.extract_us = start_extract.elapsed().as_micros();
        debug!("history: {}", redaction_text);

        let response = self
            .caller
            .call(
                self.model.as_str(),
                self.system_prompt.as_str(),
                &redaction_text,
                stat,
            )
            .await;

        if let Some(r) = response {
            //replace redacted strings
            let redacts = self.redactions(&r);
            if !redacts.is_empty() {
                let start_upd = Instant::now();
                self.update_log(&mut log, &redacts);
                stat.upd_us = start_upd.elapsed().as_micros();
            }
            debug!("Save result to {}", file_name);
            let start_save = Instant::now();
            self.saver.save(log, file_name).await;
            stat.storage_micros = start_save.elapsed().as_micros();
            stat.storage = self.saver.get_name();
            return ProcessResult::Ok;
        }
        error!("LLM failure");
        ProcessResult::Error
    }
    /// log in debug mode
    fn debug(label: &str, payload: &Vec<u8>) {
        if tracing::enabled!(Level::DEBUG) {
            if let Ok(text) = str::from_utf8(payload) {
                debug!("{} (non-UTF-8 bytes): {}", label, text);
            } else {
                debug!("{} (non-UTF-8 bytes): {:?}", label, payload);
            }
        }
    }
    /// log error in debug mode with more details
    fn error(label: &str, payload: &Vec<u8>) {
        if tracing::enabled!(Level::DEBUG) {
            if let Ok(text) = str::from_utf8(payload) {
                error!("{} (non-UTF-8 bytes): {}", label, text);
            } else {
                error!("{} (non-UTF-8 bytes): {:?}", label, payload);
            }
        }
    }

    /// get content from llm response
    pub fn extract_content(value: &Value) -> Option<&str> {
        if let Some(s) = value["choices"][0]["message"]["content"].as_str() {
            return Some(s);
        }
        error!(
            "Missing expected JSON fields: {} in response: {}",
            ".choices[0].message.content", value
        );
        None
    }
}
