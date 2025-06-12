use crate::connector::Connector;
use crate::env_vars::Cfg;
use crate::init::Init;
use crate::llm_work::llm_caller::LLmCaller;
use crate::storage::s3_saver::S3Saver;

use crate::list_env::list_env;

use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::logging;
use crate::redact_consumer::RedactConsumer;
use crate::worker_pool::WorkerPool;
use crate::worker_pool::event_counter::MinuteCounter;
use async_channel::bounded;
use async_nats::jetstream::Message;
use dotenv::dotenv;
use std::sync::Arc;

use crate::storage::get_bucket::get_bucket;
use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3helper::S3Helper;
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

        list_env();

        let cfg = Cfg::from_env();
        logging::init_log(cfg.log_level.clone());
        let connector = Connector::new(cfg.clone()).await;
        let llm_caller = LLmCaller {
            endpoint: "".to_string(),
            model: "".to_string(),
            bearer: None,
            client: Default::default(),
        };
        let shared_llm_caller = Arc::new(llm_caller);

        let system_prompt = crate::llm_work::prompt::prompt(
            &cfg.system_prompt_location, //
        );
        let bucket = get_bucket(cfg.aggregator_sessions_log_url.as_str()).unwrap();
        let access_key: Option<String> = cfg.aws_access_key_id.map(|v| v.get_string());
        let secret_key: Option<String> = cfg.aws_secret_access_key.map(|v| v.get_string());
        let access_token: Option<String> = cfg.aws_access_token.map(|v| v.get_string());

        let s3ctx = S3Ctx::new(
            bucket.clone(),
            cfg.aws_region_s3,
            access_key,
            secret_key,
            access_token,
            None,
        )
        .await;

        let s3helper = S3Helper::new(s3ctx.unwrap());
        let s3saver = Arc::new(S3Saver { s3helper, bucket });
        let processor = LlmLogProcessor::new(
            shared_llm_caller,
            system_prompt, //
            cfg.llm_model,
            s3saver,
        );
        let llm_log_processor = Arc::new(processor);

        let (snd, rcv) = bounded::<Message>(
            cfg.redact_max_tasks, //
        );

        let redact_consumer = RedactConsumer::new(
            &connector, //
            snd,
        )
        .await;

        let counter = MinuteCounter::new();
        let _pool = WorkerPool {
            size: cfg.redact_max_tasks,
            receiver: rcv,
            counter,
            llm_log_processor,
        };
        Starter { redact_consumer }
    }
}

impl Init for Starter {
    fn init(&self) -> &Self {
        self
    }
    fn start(&self) {}
}
