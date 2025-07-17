/// starter builds and starts services
use crate::config::env_vars::Cfg;
use crate::llm_work::llm_caller::LLmCaller;
use crate::llm_work::llm_log_processor::LlmLogProcessor;
use crate::llm_work::prompt::read_prompt;
use crate::mq::connector::Connector;
use crate::mq::redact_consumer::RedactConsumer;
use crate::mq::stream_admin::StreamAdmin;
use crate::mq::upd_redact_stream::update_redact_stream;
use crate::probe::http_probe::HealthProbe;
use crate::probe::toggle::Toggle;
use crate::storage::saver_factory::get_saver;
use crate::util::init::Init;
use crate::util::logging::init_log;
use crate::worker_pool::WorkerPool;
use async_channel::bounded;
use async_nats::jetstream::Message;
use async_trait::async_trait;
use dotenv::dotenv;
use num_cpus;
use std::sync::Arc;
use std::time::Duration;
use tokio::signal;
use tokio::time::timeout;
use tracing::info;

/// starter component to build and start services
pub struct Starter {
    /// create or update subjects and stream
    pub admin: StreamAdmin,
    /// consumer for incoming messages
    pub redact_consumer: RedactConsumer,
    /// workers for parallel processing
    pub worker_pool: WorkerPool,
    /// app config env vars
    pub cfg: Cfg,
    /// k8s/docker probe
    pub probe: HealthProbe,
}

/// starter methods
impl Starter {
    /// constructor
    pub async fn new() -> Self {
        info!("Create application.");
        color_backtrace::install();
        dotenv().ok();

        let cfg = Cfg::from_env();
        init_log(Some(cfg.log_level.as_str()));
        cfg.pretty();

        let s3toggle = Toggle::new("s3");

        let nats_toggle = Toggle::new("nats");
        let connector = Connector::new(&cfg, Some(&nats_toggle)).await;
        let saver = get_saver(&cfg, s3toggle.clone()).await;

        let probe = HealthProbe::new(
            vec![s3toggle, nats_toggle], //
            cfg.redact_probe_port,
        );

        let llm_caller = LLmCaller::new(
            &cfg.llm_url,
            &cfg.llm_model,
            Some(&cfg.llm_token.get_string()),
            cfg.llm_cache,
            cfg.llm_cache_sleep_millis,
        );
        let shared_llm_caller = Arc::new(llm_caller);

        let system_prompt = read_prompt(&cfg.system_prompt_location, true);

        let processor =
            LlmLogProcessor::new(shared_llm_caller, system_prompt, &cfg.llm_model, saver);
        let llm_log_processor = Arc::new(processor);

        let mut max_tasks = cfg.redact_max_tasks;
        //
        if max_tasks == 0 {
            max_tasks = num_cpus::get();
            info!("Number of max tasks not set, use number of cores: {max_tasks}");
        }

        let (snd, rcv) = bounded::<Message>(max_tasks);

        let admin = StreamAdmin::new(&connector);

        let redact_consumer = RedactConsumer::new(&connector, snd);

        let worker_pool = WorkerPool {
            size: max_tasks,
            receiver: rcv,
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

    /// handles ctrl+c
    async fn ctrl_c() {
        info!("Press Ctrl+C to stop...");
        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");
    }
}
/// starts the application services
#[async_trait]
impl Init for Starter {
    /// starts the services
    async fn start(&mut self) {
        let _ = self.probe.start().await;

        update_redact_stream(&self.admin, &self.cfg).await;

        let consumer_stop = self.redact_consumer.start(&self.cfg).await;

        self.worker_pool.start();

        Self::ctrl_c().await;
        info!("Stop application");

        let result = timeout(
            Duration::from_secs(10), // wait no longer than
            async {
                tokio::join!(
                    self.redact_consumer.stop(consumer_stop),
                    self.worker_pool.stop(),
                    self.probe.stop(),
                )
            },
        )
        .await;

        if let Err(e) = result {
            info!("Graceful stop timed out : {e}");
        } else {
            info!("Application stopped");
        }
    }
}
