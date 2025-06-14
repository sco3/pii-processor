use crate::config::env_vars::Cfg;
use crate::llm_work::llm_caller::LLmCaller;
use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::prompt::read_prompt;
use crate::mq::admin::StreamAdmin;
use crate::mq::connector::Connector;
use crate::mq::redact_consumer::RedactConsumer;
use crate::probe::http_probe::HealthProbe;
use crate::probe::toggle::Toggle;
use crate::storage::saver_factory::get_saver;
use crate::util::init::Init;
use crate::util::logging::init_log;
use crate::worker_pool::WorkerPool;
use crate::worker_pool::event_counter::MinuteCounter;

use crate::mq::upd_redact_stream::update_redact_stream;
use async_channel::bounded;
use async_nats::jetstream::Message;
use async_trait::async_trait;
use dotenv::dotenv;
use std::sync::Arc;

use tokio::signal;

use tracing::info;

pub struct Starter {
    pub admin: StreamAdmin,
    pub redact_consumer: RedactConsumer,
    pub worker_pool: WorkerPool,
    pub cfg: Cfg,
    pub probe: HealthProbe,
}

impl Starter {
    pub async fn new() -> Self {
        info!("Create application.");
        color_backtrace::install();
        dotenv().ok();

        let cfg = Cfg::from_env();
        init_log(Some(cfg.log_level.as_str()));
        cfg.pretty();

        let s3toggle = Toggle::new("s3");

        let probe = HealthProbe::new(vec![s3toggle.clone()], cfg.redact_probe_port);

        let connector = Connector::new(cfg.clone()).await;

        let llm_caller = LLmCaller::new(
            cfg.llm_url.clone(),
            cfg.llm_model.clone(),
            Some(cfg.llm_token.get_string()),
            cfg.redact_cache_enabled,
        );
        let shared_llm_caller = Arc::new(llm_caller);

        let system_prompt = read_prompt(&cfg.system_prompt_location);
        let saver = get_saver(&cfg, s3toggle).await;

        let processor = LlmLogProcessor::new(
            shared_llm_caller,
            system_prompt,
            cfg.llm_model.clone(),
            saver,
        );
        let llm_log_processor = Arc::new(processor);

        let (snd, rcv) = bounded::<Message>(cfg.redact_max_tasks);

        let admin = StreamAdmin::new(&connector);

        let redact_consumer = RedactConsumer::new(&connector, snd).await;

        let counter = MinuteCounter::new();

        let worker_pool = WorkerPool {
            size: cfg.redact_max_tasks,
            receiver: rcv,
            counter,
            llm_log_processor,
            handlers: Vec::new(),
        };

        Starter {
            admin,
            redact_consumer,
            worker_pool,
            cfg,
            probe,
        }
    }

    async fn ctrl_c() {
        info!("Press Ctrl+C to stop...");
        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
    }
}

#[async_trait]
impl Init for Starter {
    async fn start(&mut self) {
        let probe_start = self.probe.start().await;

        let cfg = self.cfg.clone();

        update_redact_stream(&self.admin, &cfg).await;

        let handle = self.redact_consumer.start(&cfg).await;

        self.worker_pool.start().await;

        Self::ctrl_c().await;

        info!("Stop application");
        let _ = probe_start.stop_tx.send(());

        self.redact_consumer.stop(handle).await;
        self.worker_pool.stop().await;
    }
}
