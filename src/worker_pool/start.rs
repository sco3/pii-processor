use crate::worker_pool::WorkerPool;
use tokio::spawn;

impl WorkerPool {
    pub async fn start(&self) {
        for _ in 0..self.size {
            let recv = self.receiver.clone();
            spawn(async move {
                Self::serve_messages(recv).await;
            });
        }
    }
}
