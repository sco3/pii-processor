use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::mq::ack::Ack;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use std::collections::HashMap;

use std::sync::Arc;
use time::OffsetDateTime;
use tracing::{debug, error, info};

/// Represents statistics for a message processing operation.
#[derive(Debug)]
pub struct Stat {
    /// The UTC timestamp when the processing of the message started.
    pub start: OffsetDateTime,
    /// The sequence number of the NATS message.
    pub seq: u64,
    /// The UTC timestamp when the NATS message was published.
    pub published: OffsetDateTime,
    /// time for building and sending request to LLM
    pub build_and_call: u128,
    /// time for llm rest api call
    pub send_micros: u128,
    /// informative headers from rest api calls
    pub extra_info: HashMap<String, String>,
    /// storage kind: s3, fs etc
    pub storage: String,
    /// storage write time
    pub storage_micros: u128,
    /// cache hit since message processing start
    pub cache_hit_micros: u128,
    /// cache get duration
    pub cache_get_micros: u128,
    /// current cache len
    pub cache_len: u64,
}

impl Stat {
    /// Creates instance with
    #[must_use]
    pub fn new() -> Self {
        Stat {
            start: OffsetDateTime::now_utc(),
            ..Default::default()
        }
    }
}
impl Default for Stat {
    fn default() -> Self {
        Stat {
            start: OffsetDateTime::UNIX_EPOCH,
            seq: 0,
            published: OffsetDateTime::UNIX_EPOCH,
            build_and_call: 0,
            send_micros: 0,
            extra_info: HashMap::new(),
            storage: String::new(),
            storage_micros: 0,
            cache_hit_micros: 0,
            cache_get_micros: 0,
            cache_len: 0,
        }
    }
}

impl WorkerPool {
    /// Serves incoming messages from a receiver channel to an LLM processor.
    pub async fn serve_message(
        recv: Receiver<Message>, //
        processor: Arc<LlmLogProcessor>,
        worker_id: usize,
    ) {
        while let Ok(msg) = recv.recv().await {
            // get sequence
            let mut seq = 0;
            let mut published = OffsetDateTime::UNIX_EPOCH;
            if let Ok(i) = msg.info() {
                seq = i.stream_sequence;
                published = i.published;
            }
            let mut stat = Stat {
                start: OffsetDateTime::now_utc(),
                seq,
                published,
                ..Default::default()
            };

            Self::process_message(&processor, worker_id, &msg, seq, published, &mut stat).await;
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
        debug!("Worker {} start seq: {}", worker_id, info.0);
        debug!("Message: {:?} {:?}", msg.payload, msg.headers);

        Stat {
            start,
            ..Default::default()
        }
    }
    /// Acknowledges a NATS message.
    pub async fn ack(ack: &dyn Ack) {
        if let Err(e) = ack.ack().await {
            error!("Acknowledge: {}", e);
        }
    }
    /// Logs the completion of message processing and calculates elapsed times.
    pub fn log_finish(stat: &Stat, published: OffsetDateTime) {
        let now = OffsetDateTime::now_utc();
        let since_publish = now - published;
        let took = now - stat.start;

        // let allocated = jemalloc_ctl::epoch::advance()
        //     .and_then(|_| jemalloc_ctl::stats::allocated::read())
        //     .unwrap();

        info!(
            "Finish in {} us since published: {} us rest call: {} us cache hit: {} \
            cache get: {} {} save: {} us extra: {:?}",
            took.whole_microseconds(),
            since_publish.whole_microseconds(),
            stat.send_micros,
            stat.cache_hit_micros,
            stat.cache_get_micros,
            stat.storage,
            stat.storage_micros,
            stat.extra_info,
        );
    }
}
