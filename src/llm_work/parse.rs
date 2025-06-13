use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::data::session_log_models::SessionLog;

use std::cmp::min;
use tracing::{debug, error};

impl LlmLogProcessor {
    pub fn parse(payload: Vec<u8>) -> Option<SessionLog> {
        match serde_json::from_slice::<SessionLog>(&payload) {
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
