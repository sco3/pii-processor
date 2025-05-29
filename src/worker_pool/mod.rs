/// message processing methods for worker pool
pub mod process_message;
/// serving methods for worker pool
pub mod serve;
/// star methods for worker pool
pub mod start;

use crate::llm_work::llm_log_processor::LlmLogProcessor;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Manages a pool of workers for processing messages.
pub struct WorkerPool {
    /// The number of workers in the pool.
    pub size: usize,
    /// The channel from which workers receive NATS messages.
    pub receiver: Receiver<Message>,
    /// An atomically reference-counted LLM log processor instance shared among workers.
    pub llm_log_processor: Arc<LlmLogProcessor>,
    /// A collection of `JoinHandle`s for managing the lifecycle of each worker task.
    pub handlers: Vec<JoinHandle<()>>,
}
