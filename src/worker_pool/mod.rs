pub mod event_counter;
pub mod header;
mod serve;
mod start;

use crate::llm_work::llm_log_processor::LlmLogProcessor;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use event_counter::MinuteCounter;
use std::sync::Arc;

pub struct WorkerPool {
    pub size: usize,
    pub receiver: Receiver<Message>,
    pub counter: MinuteCounter,
    pub llm_log_processor: Arc<LlmLogProcessor>,
}
