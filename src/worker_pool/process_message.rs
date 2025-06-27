use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::process_result::ProcessResult;
use crate::mq::nats_ack::NatsAck;
use crate::mq::session_log_header::SESSION_LOG_HEADER;
use crate::worker_pool::WorkerPool;
use crate::worker_pool::serve::Stat;
use async_nats::jetstream::Message;
use std::sync::Arc;
use std::time::Instant;

use tracing::{debug, error, instrument};

impl WorkerPool {
    /// Processes a single NATS message using the LLM log processor.
    #[instrument(name = "", level = "info", skip(stat, msg, processor))]
    pub async fn process_message(
        processor: &Arc<LlmLogProcessor>, //
        worker_id: usize,
        msg: &Message,
        seq: u64,
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
                Self::ack(&NatsAck::from(msg)).await;
                return;
            }
        };
        let payload = msg.payload.to_vec();
        match processor.process(payload, session_log_name, stat).await {
            ProcessResult::Ok => {
                debug!("PII processing finished: {session_log_name}");
                let start_ack = Instant::now();
                Self::ack(&NatsAck::from(msg)).await;
                stat.ack_micros = start_ack.elapsed().as_micros();
            }
            ProcessResult::ParseError => {
                error!("Failed to parse, acknowledge {}", session_log_name);
                Self::ack(&NatsAck::from(msg)).await;
            }
            ProcessResult::Error => {
                error!("Failed to process: {}", session_log_name);
            }
        }
        Self::log_finish(stat);
    }
}
