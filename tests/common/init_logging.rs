use ductaper::logging;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub fn init_tracing() {
    logging::LOG_INIT.call_once(|| {
        let filter = EnvFilter::new("debug,async_nats=warn");

        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .with_env_filter(filter)
            .with_test_writer()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            println!("Sorry. Tracing already initialized: {}", e);
        }
    });
}
