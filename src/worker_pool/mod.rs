mod serve;
mod start;

use async_channel::Receiver;
use async_nats::jetstream::Message;
use crate::event_counter::MinuteCounter;

pub struct WorkerPool {
    pub size: usize,
    pub receiver: Receiver<Message>,
    pub counter:MinuteCounter,
}
