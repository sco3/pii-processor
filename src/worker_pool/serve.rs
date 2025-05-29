use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, info};

/// Represents statistics for a message processing operation.
pub struct Stat {
    /// The UTC timestamp when the processing of the message started.
    pub start: OffsetDateTime,
    /// The sequence number of the NATS message.
    pub seq: u64,
    /// The UTC timestamp when the NATS message was published.
    pub published: OffsetDateTime,
}

impl WorkerPool {
    /// Serves incoming messages from a receiver channel to an LLM processor.
    pub async fn serve_message(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
        worker_id: usize,
    ) {
        while let Ok(msg) = recv.recv().await {
            Self::process_message(&processor, worker_id, &msg).await;
        }

        debug!("Channel closed. Exit worker: {}", worker_id);
    }

    /// Logs the start of message processing and captures initial statistics.
    pub fn log_start(worker_id: usize, msg: &Message) -> Stat {
        let start = OffsetDateTime::now_utc();
        let info = match msg.info() {
            Ok(info) => (info.stream_sequence, info.published),
            Err(_) => (0, OffsetDateTime::UNIX_EPOCH), // 0,0
        };
        info!("Worker {} start seq: {}", worker_id, info.0);
        debug!("Message: {:?} {:?}", msg.payload, msg.headers);

        Stat {
            start,
            seq: info.0,
            published: info.1,
        }
    }
    /// Acknowledges a NATS message.
    pub async fn ack(msg: &Message) {
        if let Err(e) = msg.ack().await {
            error!("Acknowledge: {}", e);
        }
    }
    /// Logs the completion of message processing and calculates elapsed times.
    pub fn log_finish(worker_id: usize, stat: &Stat) {
        let now = OffsetDateTime::now_utc();
        let since_publish = now - stat.published;
        let took = now - stat.start;
        // let allocated = jemalloc_ctl::epoch::advance()
        //     .and_then(|_| jemalloc_ctl::stats::allocated::read())
        //     .unwrap();

        info!(
            "Worker {} finish seq: {} took: {} us since published: {} us",
            worker_id,
            stat.seq,
            took.whole_microseconds(),
            since_publish.whole_microseconds(),
        );
    }
}
