use crate::connector::Connector;
use crate::env_vars::Cfg;
use crate::init::Init;
use crate::llm_caller::LLmCaller;
use crate::llm_handler::LlmHandler;
use std::env;

use crate::llm_work::LlmLogProcessor;
use crate::logging;
use crate::redact_consumer::RedactConsumer;
use crate::worker_pool::WorkerPool;
use async_channel::bounded;
use async_nats::jetstream::Message;
use dotenv::dotenv;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

pub struct Starter {
    //pub cfg: Cfg,
    pub redact_consumer: RedactConsumer,
}

impl Starter {
    pub async fn new() -> Self {
        info!("Start");
        color_backtrace::install();
        dotenv().ok();
        for (key, value) in env::vars() {
            println!("{key}: {value}");
        }

        let cfg = Cfg::from_env();
        logging::init_log(cfg.log_level.clone());
        let connector = Connector::new(cfg.clone()).await;
        let llm_caller = LLmCaller {
            endpoint: "".to_string(),
            model: "".to_string(),
            bearer: None,
            client: Default::default(),
        };
        let shared_llm_caller = Arc::new(Mutex::new(llm_caller));
        let llm_log_processor = LlmLogProcessor::new(
            cfg.system_prompt_location, //
            shared_llm_caller,
        );
        let llm_handler = LlmHandler::new(llm_log_processor);
        //let shared_llm_handler = Arc::new(Mutex::new(llm_handler));
        let (snd, rcv) = bounded::<Message>(cfg.redact_max_tasks);

        let redact_consumer = RedactConsumer::new(
            connector, //
            snd,
        )
        .await;

        let pool = WorkerPool {
            size: cfg.redact_max_tasks,
            receiver: rcv,
        };
        Starter { redact_consumer }
    }
}

impl Init for Starter {
    fn init(&self) -> &Self {
        self
    }
    fn start(&self) -> &Self {
        self
    }
}
