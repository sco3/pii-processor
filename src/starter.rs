use crate::env_vars::Cfg;
use crate::init::Init;
use crate::logging;
use tracing::{debug, error, info, warn};

pub struct Starter {
    pub cfg: Cfg,
}

impl Starter {
    pub fn new(cfg: Option<Cfg>) -> Self {
        let cfg = cfg.unwrap_or_else(Cfg::from_env);
        Starter { cfg }
    }
}

impl Init for Starter {
    fn init(&self) {
        let cfg = Cfg::from_env();

        let log_level = "debug".to_string();

        logging::init_log(&log_level);

        info!("Config: {:?}", cfg.clone());
        error!("Log level set to: {}", log_level);
        debug!("Log level set to: {}", log_level);
        warn!("Log level set to: {}", log_level);
    }
    fn start(&self) {}
}
