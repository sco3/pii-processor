use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::mq::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use async_nats::jetstream::Message;
use std::sync::Arc;
use tracing::{debug, error, info};

impl WorkerPool {
    /// Processes a single NATS message using the LLM log processor.
    pub async fn process_message(
        processor: &Arc<LlmLogProcessor>, //
        worker_id: usize,
        msg: &Message,
    ) {
        let start = Self::log_start(worker_id, msg);
        debug!("Message: {:?} {:?}", msg.payload, msg.headers);

        let session_log_name: &str = match msg
            .headers
            .as_ref()
            .and_then(|headers_map| headers_map.get(SESSION_LOG_HEADER))
            .map(async_nats::HeaderValue::as_str)
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
                return;
            }
        };
        match processor
            .process(msg.payload.to_vec(), session_log_name)
            .await
        {
            ProcessResult::Ok(metrics) => {
                info!("PII processing finished: {session_log_name} {metrics}",);
                Self::ack(msg).await;
            }
            ProcessResult::ParseError(_) => {
                error!("Failed to parse, acknowledge {}", session_log_name);
                Self::ack(msg).await;
            }
            ProcessResult::Error(_) => {
                error!("Failed to process: {}", session_log_name);
            }
        }
        Self::log_finish(worker_id, &start);
    }
}
