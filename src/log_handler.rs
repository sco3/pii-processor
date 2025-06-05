use async_nats::jetstream::Message;

pub trait LogHandler {
    fn handle(&mut self, arg: Message) -> bool;
}
