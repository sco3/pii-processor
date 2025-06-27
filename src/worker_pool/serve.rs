use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::mq::ack::Ack;
use crate::worker_pool::WorkerPool;
use async_channel::Receiver;

use async_nats::jetstream::Message;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use time::OffsetDateTime;
use tracing::{debug, error, info};

/// Represents statistics for a message processing operation.
#[derive(Debug)]
pub struct Stat {
    /// start of message processing for tracking
    pub start_instant: Instant,
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
    /// ack time in micros
    pub ack_micros: u128,
    /// message parse micros
    pub parse_us: u128,
    /// extract plain text from session log us
    pub extract_us: u128,
    /// update log back after redacting
    pub upd_us: u128,
    /// request build us
    pub build_us: u128,
    /// response parse us
    pub parse_resp_us: u128,
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
            start_instant: Instant::now(),
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
            ack_micros: 0,
            parse_us: 0,
            extract_us: 0,
            upd_us: 0,
            build_us: 0,
            parse_resp_us: 0,
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
            let start_processing = OffsetDateTime::now_utc();
            // get sequence
            let mut seq = 0;
            let mut published = OffsetDateTime::UNIX_EPOCH;
            if let Ok(i) = msg.info() {
                seq = i.stream_sequence;
                published = i.published;
            }
            let mut stat = Stat {
                start: start_processing,
                seq,
                published,
                ..Default::default()
            };

            Self::process_message(&processor, worker_id, &msg, seq, &mut stat).await;
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
    pub fn log_finish(stat: &Stat) {
        let now = OffsetDateTime::now_utc();
        let since_publish = now - stat.published;
        let took = stat.start_instant.elapsed().as_micros();

        // let allocated = jemalloc_ctl::epoch::advance()
        //     .and_then(|_| jemalloc_ctl::stats::allocated::read())
        //     .unwrap();
        // time spent besides LLM call and Storage saving

        let delta = took
            - stat.build_us
            - stat.send_micros
            - stat.cache_hit_micros
            - stat.parse_resp_us
            - stat.upd_us
            - stat.storage_micros
            - stat.parse_us
            - stat.extract_us
            - stat.ack_micros;
        info!(
            "Total: {took} μs since_published: {} μs parse: {} μs  extract: {} μs req_build: {} \
            μs rest_call: {} μs cache_hit: {} μs inc_cache_get: {} μs \
            resp_parse: {} μs update: {} μs {}_save: {} μs ack: {} μs \
            Δ: {} μs extra: {:?}",
            since_publish.whole_microseconds(),
            stat.parse_us,
            stat.extract_us,
            stat.build_us,
            stat.send_micros,
            stat.cache_hit_micros,
            stat.cache_get_micros,
            stat.parse_resp_us,
            stat.upd_us,
            stat.storage,
            stat.storage_micros,
            stat.ack_micros,
            delta,
            stat.extra_info,
        );
    }
}
