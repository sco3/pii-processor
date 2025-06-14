use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::mq::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, info};

struct Stat {
    start: OffsetDateTime,
    seq: u64,
    published: OffsetDateTime,
}

impl WorkerPool {
    pub async fn serve_message(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
        worker_id: usize,
    ) {
        while let Ok(msg) = recv.recv().await {
            let start = Self::log_start(worker_id, &msg);
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
                    Self::ack(&msg).await;
                }
                ProcessResult::ParseError => {
                    error!("Failed to parse, acknowledge {}", session_log_name);
                    Self::ack(&msg).await;
                }
                ProcessResult::Error => {
                    error!("Failed to process: {}", session_log_name);
                }
            }
            Self::log_finish(worker_id, start);
        }
    }

    fn log_start(worker_id: usize, msg: &Message) -> Stat {
        let start = OffsetDateTime::now_utc();
        let info = match msg.info() {
            Ok(info) => (info.stream_sequence, info.published),
            Err(_) => (0, OffsetDateTime::from_unix_timestamp(0).unwrap()),
        };
        info!("Worker {} start seq: {}", worker_id, info.0);
        debug!("Message: {:?} {:?}", msg.payload, msg.headers);

        Stat {
            start,
            seq: info.0,
            published: info.1,
        }
    }

    async fn ack(msg: &Message) {
        if let Err(e) = msg.ack().await {
            error!("Acknowledge: {}", e)
        }
    }

    fn log_finish(worker_id: usize, stat: Stat) {
        let now = OffsetDateTime::now_utc();
        let since_publish = now - stat.published;
        let took = now - stat.start;
        info!(
            "Worker {} finish seq: {} took: {} us since published: {} us",
            worker_id,
            stat.seq,
            took.whole_microseconds(),
            since_publish.whole_microseconds(),
        );
    }
}
