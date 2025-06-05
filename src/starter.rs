use crate::env_vars::Cfg;
use crate::init::Init;
use crate::llm_handler::LlmHandler;
use crate::logging;
use crate::redact_consumer::RedactConsumer;
use dotenv::dotenv;
use tracing::info;

pub struct Starter {
    pub cfg: Cfg,
    pub redact_consumer: RedactConsumer,
}

impl Starter {
    pub async fn new(cfg: Option<Cfg>) -> Self {
        dotenv().ok();
        let cfg = cfg.unwrap_or_else(Cfg::from_env);
        let llm_handler = LlmHandler {};
        let redact_consumer = RedactConsumer::new(
            &cfg, //
            Box::new(llm_handler),
        )
        .await;
        Starter {
            cfg,
            redact_consumer,
        }
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
