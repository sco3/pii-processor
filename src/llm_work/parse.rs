use crate::data::session_log_models::SessionLog;
use crate::llm_work::llm_log_processor::LlmLogProcessor;

use std::cmp::min;
use tracing::{debug, error};

impl LlmLogProcessor {
    /// parse nats message payload to session log model/structure
    pub fn parse(payload: &[u8]) -> Option<SessionLog> {
        match serde_json::from_slice::<SessionLog>(payload) {
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
