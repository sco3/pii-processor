mod start;
mod serve;

use async_channel::Receiver;
use async_nats::jetstream::Message;

pub struct WorkerPool{
    size:u16,
    receiver: Receiver<Message>, 
}