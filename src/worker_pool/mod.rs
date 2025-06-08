mod serve;
mod start;
pub mod event_counter;
pub mod header;

use async_channel::Receiver;
use async_nats::jetstream::Message;
use event_counter::MinuteCounter;

pub struct WorkerPool {
    pub size: usize,
    pub receiver: Receiver<Message>,
    pub counter:MinuteCounter,
}
