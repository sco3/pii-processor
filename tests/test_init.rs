use crate::common::init_cfg::{AGGREGATOR_SESSIONS_LOG_URL, APPLICATION, LLM_MODEL, TENANT};
use redact::starter::Starter;
use redact::util::exit_codes::ExitCode;
use std::env;

mod common;

#[allow(dead_code)]
async fn test_init() {
    unsafe {
        env::set_var(TENANT, "TENANT");
        env::set_var(APPLICATION, "APPLICATION");
        env::set_var(LLM_MODEL, "nova");
        env::set_var(AGGREGATOR_SESSIONS_LOG_URL, "s3://test");
    }

    let _starter = Starter::new().await;

    assert_eq!(0, ExitCode::Success.code());
}
