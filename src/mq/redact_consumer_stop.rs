use crate::mq::redact_consumer::RedactConsumer;
use crate::mq::redact_consumer_start::ConsumerStop;
use tracing::{error, info};

impl RedactConsumer {
    /// stops the consumer
    pub async fn stop(&self, stop: ConsumerStop) {
        info!("Stop consumer");
        // let flag = self.get_run_flag();
        // flag.store(false, Ordering::Relaxed);
        self.sender.close();

        if let Err(()) = stop.stop_tx.send(()) {
            error!("Failed to send stop signal to consumer.");
        }
        let _ = stop.join_handle.await;
    }
}
