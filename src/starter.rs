use crate::connector::Connector;
use crate::env_vars::Cfg;
use crate::init::Init;
use crate::llm_caller::LLmCaller;
use crate::llm_handler::LlmHandler;

use crate::llm_work::LlmLogProcessor;
use crate::logging;
use crate::redact_consumer::RedactConsumer;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct Starter {
    //pub cfg: Cfg,
    pub redact_consumer: RedactConsumer,
}

impl Starter {
    pub async fn new(cfg: Option<Cfg>) -> Self {
        dotenv().ok();
        let cfg = cfg.unwrap_or_else(Cfg::from_env);
        let connector = Connector::new(cfg.clone()).await;
        let llm_caller = LLmCaller {
            endpoint: "".to_string(),
            model: "".to_string(),
            bearer: None,
            client: Default::default(),
        };
        let shared_llm_caller = Arc::new(Mutex::new(llm_caller));
        let llm_log_processor = LlmLogProcessor::new(cfg, shared_llm_caller);
        let llm_handler = LlmHandler::new(llm_log_processor);
        let shared_llm_handler = Arc::new(Mutex::new(llm_handler));
        let redact_consumer = RedactConsumer::new(
            connector, //
            shared_llm_handler,
        )
        .await;
        Starter { redact_consumer }
    }
}

impl Init for Starter {
    fn init(&self) -> &Self {
        let cfg = Cfg::from_env();

        logging::init_log(cfg.log_level.clone());

        info!("Config: {:?}", cfg.clone());

        self
    }
    fn start(&self) -> &Self {
        self
    }
}
