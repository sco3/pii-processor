use crate::llm_work::LlmLogProcessor;
use crate::log_handler::LogHandler;
use async_nats::jetstream::Message;
use async_trait::async_trait;

pub struct LlmHandler {
    processor: LlmLogProcessor,
}

impl LlmHandler {
    pub fn new(processor: LlmLogProcessor) -> Self {
        LlmHandler { processor }
    }
}
#[async_trait]
impl LogHandler for LlmHandler {
    async fn handle(&mut self, msg: Message) -> bool {
        let payload: &[u8] = msg.payload.as_ref();
        self.processor.process(payload).await
    }

    fn cnt(&self) -> i32 {
        0
    }
}
