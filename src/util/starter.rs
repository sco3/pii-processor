use crate::config::env_vars::Cfg;
use crate::llm_work::llm_caller::LLmCaller;
use crate::mq::connector::Connector;
use crate::storage::s3_saver::S3Saver;
use crate::util::init::Init;
use std::process::exit;

use crate::llm_work::llm_log_processor::LlmLogProcessor;

use crate::llm_work::prompt::read_prompt;
use crate::mq::redact_consumer::RedactConsumer;
use crate::probe::http_probe::HealthProbe;
use crate::probe::toggle::Toggle;
use crate::storage::get_bucket::get_bucket;
use crate::storage::s3ctx::S3Ctx;
use crate::storage::s3helper::S3Helper;
use crate::util::exit_codes::ExitCode;
use crate::util::logging::init_log;
use crate::worker_pool::WorkerPool;
use crate::worker_pool::event_counter::MinuteCounter;
use async_channel::bounded;
use async_nats::jetstream::Message;
use async_trait::async_trait;
use dotenv::dotenv;
use std::sync::Arc;
use tokio;
use tokio::signal;
use tokio::sync::Mutex;
use tracing::{error, info};

pub struct Starter {
    pub redact_consumer: Arc<Mutex<RedactConsumer>>,
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
        let probe = HealthProbe::new(
            vec![s3toggle.clone()], //
            cfg.redact_probe_port,
        );

        let connector = Connector::new(cfg.clone()).await;
        let llm_caller = LLmCaller {
            endpoint: cfg.llm_url.clone(),
            model: cfg.llm_model.clone(),
            bearer: None,
            client: Default::default(),
        };
        let shared_llm_caller = Arc::new(llm_caller);

        let system_prompt = read_prompt(
            &cfg.system_prompt_location, //
        );
        let bucket = get_bucket(cfg.aggregator_sessions_log_url.as_str()).unwrap();
        let access_key: Option<String> = cfg.aws_access_key_id.clone().map(|v| v.get_string());
        let secret_key: Option<String> = cfg.aws_secret_access_key.clone().map(|v| v.get_string());
        let access_token: Option<String> = cfg.aws_access_token.clone().map(|v| v.get_string());

        let s3ctx = match S3Ctx::new(
            bucket.clone(),
            cfg.aws_region_s3.clone(),
            access_key,
            secret_key,
            access_token,
            None,
        )
        .await
        {
            Ok(ctx) => {
                s3toggle.set_ready(true);
                ctx
            }
            Err(e) => {
                error!("S3 problem: {}", e);
                exit(ExitCode::S3Error.code());
            }
        };

        let s3helper = S3Helper::new(s3ctx);
        let s3saver = Arc::new(S3Saver {
            s3helper,
            bucket,
            toggle: s3toggle,
        });
        let processor = LlmLogProcessor::new(
            shared_llm_caller,
            system_prompt, //
            cfg.llm_model.clone(),
            s3saver,
        );
        let llm_log_processor = Arc::new(processor);

        let (snd, rcv) = bounded::<Message>(
            cfg.redact_max_tasks, //
        );

        let redact_consumer = Arc::new(Mutex::new(
            RedactConsumer::new(
                &connector, //
                snd,
            )
            .await,
        ));

        let counter = MinuteCounter::new();
        let worker_pool = WorkerPool {
            size: cfg.redact_max_tasks,
            receiver: rcv,
            counter,
            llm_log_processor,
        };
        Starter {
            redact_consumer,
            worker_pool,
            cfg,
            probe,
        }
    }
}
#[async_trait]
impl Init for Starter {
    async fn start(&mut self) {
        if let Err(e) = self.probe.start().await {
            error!("Probe failed: {}", e);
            exit(ExitCode::ProbeError.code());
        }

        let consumer = Arc::clone(&self.redact_consumer);
        let cfg = self.cfg.clone();

        tokio::spawn(async move {
            let mut consumer = consumer.lock().await;
            consumer.update_stream(&cfg).await;
            if let Err(e) = consumer.subscribe(&cfg).await {
                error!("Subscription failed: {}", e);
                exit(ExitCode::NatsError.code());
            }
            consumer.serve().await;
        });

        self.worker_pool.start().await;

        signal::ctrl_c()
            .await
            .expect("Failed to listen for shutdown signal");

        info!("Stop application");
        exit(ExitCode::Success.code());
    }
}
