use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::mq::nats_ack::NatsAck;
use crate::mq::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use crate::worker_pool::serve::Stat;
use async_nats::jetstream::Message;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, instrument};

impl WorkerPool {
    /// Processes a single NATS message using the LLM log processor.
    #[instrument(name = "", level = "info", skip(stat, msg, processor, published))]
    pub async fn process_message(
        processor: &Arc<LlmLogProcessor>, //
        worker_id: usize,
        msg: &Message,
        seq: u64,
        published: OffsetDateTime,
        stat: &mut Stat,
    ) {
        debug!("Message: {seq:?} {:?} {:?}", msg.payload, msg.headers);

        let session_log_name: &str = match msg
            .headers
            .as_ref()
            .and_then(|headers| headers.get(SESSION_LOG_HEADER))
            .map(async_nats::HeaderValue::as_str)
        {
            Some(name) if !name.is_empty() => name,
            _ => {
                // None, or empty
                error!(
                    "Header '{}' not found or is empty in message. \
                    Skipping processing. Payload: {:?}", //
                    SESSION_LOG_HEADER, msg.payload
                );
                return;
            }
        };

        match processor
            .process(msg.payload.to_vec(), session_log_name, stat)
            .await
        {
            ProcessResult::Ok(metrics) => {
                debug!("PII processing finished: {session_log_name} {metrics}",);
                stat.storage = metrics.save_kind;
                stat.storage_micros = metrics.save_micros;
                Self::ack(&NatsAck::from(msg)).await;
            }
            ProcessResult::ParseError(_) => {
                error!("Failed to parse, acknowledge {}", session_log_name);
                Self::ack(&NatsAck::from(msg)).await;
            }
            ProcessResult::Error(_) => {
                error!("Failed to process: {}", session_log_name);
            }
        }
        Self::log_finish(stat, published);
    }
}
