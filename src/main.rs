use crate::env_vars::Cfg;

use tracing::{debug, error, info, warn};

mod env_vars;
mod logging;
mod secret_string;

fn main() {
    let cfg = Cfg::from_env();

    let mut log_level = cfg.log_level.clone();
    log_level = "debug".to_string();

    logging::init_log(&log_level);

    info!("Config: {:?}", cfg.clone());
    error!("Log level set to: {}", log_level);
    debug!("Log level set to: {}", log_level);
    warn!("Log level set to: {}", log_level);
}
