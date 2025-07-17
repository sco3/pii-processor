use ductaper::config::env_vars::Cfg;
use ductaper::config::secret_string::SecretString;

#[allow(dead_code)]
pub const AGGREGATOR_SESSIONS_LOG_URL: &str = "AGGREGATOR_SESSIONS_LOG_URL";
#[allow(dead_code)]
pub const NATS_URL: &str = "NATS_URL";
#[allow(dead_code)]
pub const LLM_TOKEN: &str = "LLM_TOKEN";
#[allow(dead_code)]
pub const LOG_LEVEL: &str = "LOG_LEVEL";
#[allow(dead_code)]
pub const LLM_MODEL: &str = "LLM_MODEL";
#[allow(dead_code)]
pub const TENANT: &str = "TENANT";
#[allow(dead_code)]
pub const APPLICATION: &str = "APPLICATION";
#[allow(dead_code)]
pub const LLM_CACHE: &str = "LLM_CACHE";
#[allow(dead_code)]
pub fn get_test_cfg(nats_port: u16) -> Cfg {
    Cfg {
        llm_token: SecretString::new("sk-1234"),
        log_level: "debug".to_string(),
        redact_subject: "redact".to_string(),
        queue_stream: "queue".to_string(),
        queue_stream_max_age_seconds: 3600 * 24,
        nats_url: format!("nats://localhost:{nats_port}"),
        tenant: "tenant".to_string(),
        application: "application".to_string(),
        aws_region_s3: "eu-west-1".to_string(),
        aws_s3_endpoint: None,
        aws_access_key_id: None,
        aws_secret_access_key: None,
        aws_access_token: None,
        llm_model: "nova".to_string(),
        aggregator_sessions_log_url: "s3://test".to_string(),
        system_prompt_location: "/tmp/system_prompt.txt".to_string(),
        redact_max_tasks: 8,
        llm_url: "http://localhost:4000/chat/completions".to_string(),
        redact_probe_port: 8118,
        llm_cache: false,
        llm_cache_sleep_millis: 0,
    }
}
