use crate::common::init_cfg::{AGGREGATOR_SESSIONS_LOG_URL, APPLICATION, LLM_MODEL, TENANT};
use ductaper::init::Init;
use ductaper::starter::Starter;
use std::env;

mod common;

#[tokio::test]
async fn test_init() {
    unsafe {
        env::set_var(TENANT, "TENANT");
        env::set_var(APPLICATION, "APPLICATION");
        env::set_var(LLM_MODEL, "nova");
        env::set_var(LLM_MODEL, "nova");
        env::set_var(AGGREGATOR_SESSIONS_LOG_URL, "s3://test");
    }

    let starter = Starter::new(None).await;
    starter.init();
    starter.start();
}
