use ductaper::logging;
use tracing_subscriber::{EnvFilter, FmtSubscriber};
#[allow(dead_code)]
pub fn init_tracing() {
    logging::LOG_INIT.call_once(|| {
        let filter = EnvFilter::new("debug,async_nats=warn,hyper_util=warn,S3=warn");

        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
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
