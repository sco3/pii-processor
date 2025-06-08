mod serve;
mod start;

use async_channel::Receiver;
use async_nats::jetstream::Message;

pub struct WorkerPool {
    pub size: usize,
    pub receiver: Receiver<Message>,
}
