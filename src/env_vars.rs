use crate::secret_string::SecretString;
use envy;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Cfg {
    #[serde(default = "default_nats_url")]
    pub nats_url: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,

    #[serde(default = "default_llm_token")]
    pub llm_token: SecretString,

    pub tenant: String,
    pub application: String,

    #[serde(default = "default_queue_stream")]
    pub queue_stream: String,

    #[serde(default = "default_redact_subject")]
    pub redact_subject: String,

    #[serde(default = "default_queue_stream_max_age")]
    pub queue_stream_max_age: u64,
}
fn default_queue_stream_max_age() -> u64 {
    60 * 60 * 24 // 1 day in seconds
}
fn default_llm_token() -> SecretString {
    SecretString::new("sk-1234")
}

fn default_nats_url() -> String {
    "nats://localhost:4222".to_string()
}
fn default_log_level() -> String {
    "info".to_string()
}
fn default_redact_subject() -> String {
    "redact-log".to_string()
}

fn default_queue_stream() -> String {
    "queue-stream".to_string()
}

impl Cfg {
    pub fn from_env() -> Self {
        envy::from_env::<Cfg>().unwrap_or_else(|err| {
            panic!("Failed to load configuration : {}", err);
        })
    }
}
