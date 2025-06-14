use crate::worker_pool::WorkerPool;
use std::sync::Arc;
use tokio::spawn;
use tracing::info;

impl WorkerPool {
    pub async fn start(&self) {
        for id in 0..self.size {
            let recv = self.receiver.clone();
            let processor = Arc::clone(&self.llm_log_processor);
            spawn(async move {
                Self::serve_message(recv, processor, id).await;
            });
        }
        info!("Worker pool with {} workers started.", self.size)
    }

    pub async fn stop(&self) {}
}
