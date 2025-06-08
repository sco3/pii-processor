use async_channel::bounded;
use async_nats::jetstream::Message;
use ductaper::worker_pool::WorkerPool;
use tracing::debug;

#[tokio::test]
async fn test_pool() {
    let (tx, rx) = bounded::<Message>(1);
    debug!("Tx: {:?}", tx);

    let pool = WorkerPool {
        size: 1,
        receiver: rx,
    };
    pool.start().await;
}
