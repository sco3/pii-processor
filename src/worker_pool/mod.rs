mod serve;
mod start;

use crate::llm_work::llm_log_processor::LlmLogProcessor;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use tokio::task::JoinHandle;

pub struct WorkerPool {
    pub size: usize,
    pub receiver: Receiver<Message>,
    pub llm_log_processor: Arc<LlmLogProcessor>,
    pub handlers: Vec<JoinHandle<()>>,
}
