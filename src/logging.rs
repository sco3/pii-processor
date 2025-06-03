use tracing_subscriber::{EnvFilter, fmt};
pub fn init_log(log_level: &String) {
    let filter = EnvFilter::new(log_level);
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .init();
}
