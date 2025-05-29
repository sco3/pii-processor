use async_trait::async_trait;
use ductaper::mq::ack::Ack;
//use ductaper::util::logging::init_tracing;
use ductaper::worker_pool::WorkerPool;
use std::error::Error;
use std::io;
//use tokio;
use tracing::{error, info};
use tracing_test::traced_test;

struct GoodAck {}
#[async_trait]
impl Ack for GoodAck {
    async fn ack(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("good");
        Ok(())
    }
}
struct BadAck {}
#[async_trait]
impl Ack for BadAck {
    async fn ack(&self) -> Result<(), Box<dyn Error + Send + Sync>> {
        info!("bad");
        Err(Box::new(io::Error::other("bad ack from BadAck::ack")))
    }
}

#[traced_test]
pub async fn test_ack() {
    //init_tracing();
    info!("info from test_ack");
    error!("error from test_ack");

    // Call with GoodAck - this should NOT produce an error log
    WorkerPool::ack(&GoodAck {}).await;

    // Call with BadAck - this *should* produce an error log from WorkerPool::ack
    WorkerPool::ack(&BadAck {}).await;
}
