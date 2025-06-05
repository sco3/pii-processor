use ductaper::env_vars::Cfg;
use ductaper::secret_string::SecretString;

pub fn get_test_cfg(nats_port: u16) -> Cfg {
    Cfg {
        llm_token: SecretString::new("sk-1234"),
        log_level: "debug".to_string(),
        redact_subject: "redact".to_string(),
        queue_stream: "queue".to_string(),
        queue_stream_max_age: 3600 * 24,
        nats_url: format!("nats://localhost:{nats_port}"),
        tenant: "tenant".to_string(),
        application: "application".to_string(),
        aws_region_s3: "eu-west-1".to_string(),
        aws_access_key_id: None,
        aws_secret_access_key: None,
        aws_access_token: None,
        llm_model: "nova".to_string(),
        aggregator_sessions_log_url: "s3:/test".to_string(),
    }
}
