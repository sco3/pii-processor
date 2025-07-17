use crate::config::env_vars::Cfg;
use crate::mq::connector::Connector;
use async_nats::jetstream::Context;
use std::collections::HashSet;

/// Structure to update/create stream
pub struct StreamAdmin {
    /// jet steam client
    pub jetstream: Context,
}
/// The stream update/create functions
impl StreamAdmin {
    /// constructor
    #[must_use]
    pub fn new(connector: &Connector) -> Self {
        let client = *connector.get();
        let jetstream = async_nats::jetstream::new(client.clone());
        StreamAdmin { jetstream }
    }
    /// creates full subject name from env settings
    #[must_use]
    pub fn get_full_subject(cfg: &Cfg, subject: &str) -> String {
        format!("{}.{}.{}", &cfg.tenant, &cfg.application, subject)
    }

    #[must_use]
    /// merges two vectors of string with no duplicates
    pub fn merge_unique(vec1: &[String], vec2: &[String]) -> Vec<String> {
        let mut set: HashSet<String> = HashSet::new();
        set.extend(vec1.iter().cloned());
        set.extend(vec2.iter().cloned());
        set.into_iter().collect()
    }
}
