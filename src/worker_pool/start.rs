use crate::worker_pool::WorkerPool;
use std::sync::Arc;
use tokio::spawn;

impl WorkerPool {
    pub async fn start(&self) {
        for _ in 0..self.size {
            let recv = self.receiver.clone();
            let processor = Arc::clone(&self.llm_log_processor);
            spawn(async move {
                Self::serve_messages(recv, processor).await;
            });
        }
    }
}
