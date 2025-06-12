use crate::expanduser::expand_user_path;
use crate::secret_string::SecretString;
use envy;
use serde::{Deserialize, Serialize};
use tracing::info;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Cfg {
    #[serde(default = "default_nats_url")]
    pub nats_url: String,

    #[serde(default = "default_log_level")]
    pub log_level: String,
    #[serde(default = "default_llm_token")]
    pub llm_token: SecretString,
    #[serde(default = "default_llm_url")]
    pub llm_url: String,

    pub tenant: String,
    pub application: String,

    #[serde(default = "default_queue_stream")]
    pub queue_stream: String,

    #[serde(default = "default_redact_subject")]
    pub redact_subject: String,

    #[serde(default = "default_queue_stream_max_age")]
    pub queue_stream_max_age_seconds: u64,

    #[serde(default = "default_aws_region_s3")]
    pub aws_region_s3: String,
    pub aws_access_key_id: Option<SecretString>,
    pub aws_secret_access_key: Option<SecretString>,
    pub aws_access_token: Option<SecretString>,

    pub aggregator_sessions_log_url: String,
    pub llm_model: String,
    #[serde(default = "default_system_prompt_location")]
    pub system_prompt_location: String,
    #[serde(default = "default_redact_max_tasks")]
    pub redact_max_tasks: usize,
}

fn default_llm_url() -> String {
    "http://localhost:4000/chat/completions".to_string()
}

fn default_redact_max_tasks() -> usize {
    8
}

fn default_system_prompt_location() -> String {
    expand_user_path("~/.local/system_prompt.txt")
        .to_string_lossy()
        .into_owned()
}

fn default_aws_region_s3() -> String {
    "us-east-1".to_string()
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
    "converastion-stream".to_string()
}

impl Cfg {
    pub fn from_env() -> Self {
        envy::from_env::<Cfg>().unwrap_or_else(|err| {
            panic!("Failed to load configuration : {}", err);
        })
    }

    pub fn pretty(&self) {
        if let Ok(value) = serde_json::to_value(self) {
            if let Some(obj) = value.as_object() {
                for (key, val) in obj.iter() {
                    info!("{} : {}", key.to_uppercase(), val);
                }
            }
        };
    }
}
