use crate::worker_pool::WorkerPool;
use std::sync::Arc;
use tokio::spawn;
use tracing::info;

impl WorkerPool {
    /// starts worker pool
    pub async fn start(&mut self) {
        for id in 0..self.size {
            let recv = self.receiver.clone();
            let processor = Arc::clone(&self.llm_log_processor);
            let h = spawn(async move {
                Self::serve_message(recv, processor, id).await;
            });
            let _ = &self.handlers.push(h);
        }
        info!("Worker pool with {} workers started.", self.size)
    }
    /// stops worker pool
    pub async fn stop(&mut self) {
        for handler in &mut self.handlers {
            let _ = handler.await;
        }
        info!("All workers stopped");
    }
}
