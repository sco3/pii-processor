use crate::log_handler::LogHandler;
use async_nats::jetstream::Message;

use tracing::debug;

pub struct LlmHandler;

impl LogHandler for LlmHandler {
    fn handle(&mut self, msg: Message) -> bool {
        debug!("coming soon {:?}", msg);
        true
    }
    fn cnt(&self) -> i32 {
        0
    }
}
