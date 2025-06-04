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
    fn init(&self) -> &Self {
        let cfg = Cfg::from_env();

        logging::init_log(&cfg.log_level);

        info!("Config: {:?}", cfg.clone());
        error!("Log level set to: {}", cfg.log_level);
        debug!("Log level set to: {}", cfg.log_level);
        warn!("Log level set to: {}", cfg.log_level);
        self
    }
    fn start(&self) -> &Self {
        self
    }
}
