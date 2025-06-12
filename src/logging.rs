use std::sync::Once;
use tracing::{Level, error};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub static LOG_INIT: Once = Once::new();

#[allow(dead_code)]
pub fn init_log(level_str: Option<&str>) {
    LOG_INIT.call_once(|| {
        let mut level: Level = Level::INFO;
        if let Some(s) = level_str {
            match s.parse::<Level>() {
                Ok(l) => {
                    level = l;
                }
                Err(e) => {
                    error!("Unknown level: {} {}", s, e);
                }
            }
        }

        let filter = EnvFilter::new(
            "debug,async_nats=warn,hyper_util=warn,S3=warn", //
        );

        let subscriber = FmtSubscriber::builder()
            .with_max_level(level)
            .with_env_filter(filter)
            .with_test_writer()
            .with_file(true)
            .with_line_number(true)
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            println!("Sorry. Tracing already initialized: {}", e);
        }
    });
}

pub fn init_tracing() {
    init_log(Some(Level::DEBUG.to_string().as_str()));
}
