use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use tracing::debug;

impl WorkerPool {
    pub async fn serve_messages(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
    ) {
        while let Ok(msg) = recv.recv().await {
            debug!("Message: {:?} {:?}", msg.payload, msg.headers);
            processor.process(msg.payload.to_vec()).await;
        }
    }
}
