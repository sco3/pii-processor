use crate::logging;
use std::sync::Once;
use tracing_subscriber::EnvFilter;

pub static LOG_INIT: Once = Once::new();

pub fn init_log(log_level: String) {
    logging::LOG_INIT.call_once(|| {
        let filter = EnvFilter::new(log_level);
        tracing_subscriber::fmt()
            .with_env_filter(filter)
            .with_target(true)
            .with_line_number(true)
            .init();
    });
}
