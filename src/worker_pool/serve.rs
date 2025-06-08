use crate::worker_pool::WorkerPool;
use async_channel::Receiver;
use async_nats::jetstream::Message;
use tokio::spawn;
use tracing::debug;

impl WorkerPool {
    pub async fn serve_messages(recv: Receiver<Message>) {
        while let Ok(msg) = recv.recv().await {
            debug!("Message: {:?}", msg);
        }
    }
}
