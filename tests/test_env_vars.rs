use crate::common::init_cfg::{
    AGGREGATOR_SESSIONS_LOG_URL, APPLICATION, LLM_MODEL, LLM_TOKEN, LOG_LEVEL, NATS_URL, TENANT,
};
use ductaper::config::env_vars::Cfg;
use serial_test::serial;
use std::env;
mod common;

fn set_non_defaults() {
    unsafe {
        env::set_var(TENANT, "TENANT");
        env::set_var(APPLICATION, "APPLICATION");
    }
}

#[test]
#[serial]
#[should_panic]
fn test_missing_vars() {
    set_non_defaults();
    // Ensure no environment variables are set
    unsafe {
        env::remove_var(NATS_URL);
        env::remove_var(LOG_LEVEL);
        env::remove_var(LLM_TOKEN);
        env::remove_var(AGGREGATOR_SESSIONS_LOG_URL);
    }
    // Call the method, expecting a panic
    Cfg::from_env();
}

#[test]
#[serial]
fn test_values() {
    set_non_defaults();
    unsafe {
        env::set_var(NATS_URL, "NU");
        env::set_var(LLM_TOKEN, "LLM_TOKEN");
        env::set_var(LOG_LEVEL, "DEBUG");
        env::set_var(LLM_MODEL, "LM");
        env::set_var(AGGREGATOR_SESSIONS_LOG_URL, "s3://test");
    }
    let cfg = Cfg::from_env();
    assert_eq!(cfg.nats_url, "NU");
    assert_eq!(
        cfg.llm_token.get_string(),
        "LLM_TOKEN",
        "LLM token should match"
    );
    assert_eq!(cfg.log_level, "DEBUG");
    let redacted = format!("{:?}", cfg.llm_token);
    assert_eq!("LLM_****", redacted);
}
