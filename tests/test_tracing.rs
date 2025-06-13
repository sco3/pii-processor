use ductaper::util::logging;
use tracing_subscriber::FmtSubscriber;

pub fn init_tracing() {
    logging::LOG_INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            println!("Sorry. Tracing already initialized: {}", e);
        }
    });
}
