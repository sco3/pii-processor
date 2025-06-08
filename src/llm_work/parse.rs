use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::session_log_models::SessionLogType;
use std::cmp::min;
use tracing::{debug, error};

impl LlmLogProcessor {
    pub fn parse(payload: &&[u8]) -> Option<SessionLogType> {
        match serde_json::from_slice::<SessionLogType>(payload) {
            Ok(log) => {
                debug!("Log: {:?}", log);
                Some(log)
            }
            Err(e) => {
                let head = &payload[..min(payload.len(), 80)];
                error!(
                    "Cannot parse payload {} {}",
                    String::from_utf8_lossy(head),
                    e
                );
                None
            }
        }
    }
}
