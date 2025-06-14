use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::mq::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, info};

impl WorkerPool {
    pub async fn serve_message(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
        id: usize,
    ) {
        while let Ok(msg) = recv.recv().await {
            let info = Self::info(&msg);
            info!("Worker {} start seq: {}", id, info.0);
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
            match processor
                .process(
                    msg.payload.to_vec(), //
                    session_log_name,
                )
                .await
            {
                ProcessResult::Ok => {
                    info!("PII processing finished: {}", session_log_name);
                    Self::ack(&msg, id, info).await;
                }
                ProcessResult::ParseError => {
                    error!("Failed to parse, acknowledge {}", session_log_name);
                    Self::ack(&msg, id, info).await;
                }
                ProcessResult::Error => {
                    error!("Failed to process: {}", session_log_name);
                }
            }
        }
    }

    fn info(msg: &Message) -> (u64, OffsetDateTime) {
        let seq = match msg.info() {
            Ok(info) => (info.stream_sequence, info.published),
            Err(_) => (0, OffsetDateTime::from_unix_timestamp(0).unwrap()),
        };
        seq
    }

    async fn ack(msg: &Message, id: usize, info: (u64, OffsetDateTime)) {
        let created = info.1;
        let took = OffsetDateTime::now_utc() - created;
        info!(
            "Worker {} finish seq: {} since published: {} us",
            id,
            info.0,
            took.whole_microseconds()
        );
        if let Err(e) = msg.ack().await {
            error!("Acknowledge: {}", e)
        }
    }
}
