use crate::log_handler::LogHandler;
use crate::session_log_models::SessionLogType;
use async_nats::jetstream::Message;
use serde_json;
use std::cmp::min;
use tracing::{debug, error};

pub struct LlmHandler;

impl LogHandler for LlmHandler {
    fn handle(&mut self, msg: Message) -> bool {
        let payload: &[u8] = msg.payload.as_ref();

        match serde_json::from_slice::<SessionLogType>(payload) {
            Ok(log) => {
                debug!("Log: {:?}", log);
                true
            }
            Err(e) => {
                let head = &payload[..min(payload.len(), 80)];
                error!(
                    "Cannot parse payload {} {}",
                    String::from_utf8_lossy(head),
                    e
                );
                false
            }
        }
    }
    fn cnt(&self) -> i32 {
        0
    }
}
