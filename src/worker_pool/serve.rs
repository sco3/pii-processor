use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use tracing::{debug, error};

impl WorkerPool {
    pub async fn serve_messages(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
    ) {
        while let Ok(msg) = recv.recv().await {
            debug!("Message: {:?} {:?}", msg.payload, msg.headers);
            let session_log_name: &str = match msg
                .headers
                .as_ref()
                .and_then(|headers_map| headers_map.get(SESSION_LOG_HEADER))
                .map(|header_value_string| header_value_string.as_str())
            {
                Some(name) if !name.is_empty() => name,
                _ => {
                    // None, or empty
                    error!(
                        concat!(
                            "Header '{}' not found or is empty in message.", //
                            " Skipping processing. Payload: {:?}",           //
                        ),
                        SESSION_LOG_HEADER, msg.payload
                    );
                    continue; // Skip processing without header
                }
            };
            processor
                .process(
                    msg.payload.to_vec(), //
                    session_log_name,
                )
                .await;
        }
    }
}
