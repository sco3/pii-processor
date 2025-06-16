#![deny(missing_docs)]

//! Configuration module for Ductaper application.
//!
//! This module handles loading and managing configuration settings from environment variables,
//! with sensible defaults for various services including NATS, LLM, AWS, and logging.

use crate::config::expanduser::expand_user_path;
use crate::config::secret_string::SecretString;
use envy;
use serde::{Deserialize, Serialize};
use tracing::info;

/// Main configuration structure for the application.
///
/// Contains all configurable parameters with sensible defaults.
/// Most fields can be set via environment variables following the field names in uppercase.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Cfg {
    /// NATS server URL (default: "nats://localhost:4222")
    #[serde(default = "default_nats_url")]
    pub nats_url: String,

    /// Logging level (default: "info")
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Token for LLM service (default: "sk-1234")
    #[serde(default = "default_llm_token")]
    pub llm_token: SecretString,

    /// URL for LLM service (default: "http://localhost:4000/chat/completions")
    #[serde(default = "default_llm_url")]
    pub llm_url: String,

    /// Tenant identifier (required)
    pub tenant: String,

    /// Application identifier (required)
    pub application: String,

    /// NATS stream name for queues (default: "converastion-stream")
    #[serde(default = "default_queue_stream")]
    pub queue_stream: String,

    /// Subject for redaction logs (default: "redact-log")
    #[serde(default = "default_redact_subject")]
    pub redact_subject: String,

    /// Maximum age for queue stream messages in seconds (default: 86400 = 1 day)
    #[serde(default = "default_queue_stream_max_age")]
    pub queue_stream_max_age_seconds: u64,

    /// AWS region for S3 operations (default: "us-east-1")
    #[serde(default = "default_aws_region_s3")]
    pub aws_region_s3: String,

    /// Optional AWS access key ID
    pub aws_access_key_id: Option<SecretString>,

    /// Optional AWS secret access key
    pub aws_secret_access_key: Option<SecretString>,

    /// Optional AWS session token
    pub aws_access_token: Option<SecretString>,

    /// URL for aggregator sessions log
    pub aggregator_sessions_log_url: String,

    /// LLM model identifier
    pub llm_model: String,

    /// Location of system prompt file (default: "~/.local/system_prompt.txt")
    #[serde(default = "default_system_prompt_location")]
    pub system_prompt_location: String,

    /// Maximum parallel redaction tasks (default: 8)
    #[serde(default = "default_redact_max_tasks")]
    pub redact_max_tasks: usize,

    /// Port for redaction probe server (default: 8118)
    #[serde(default = "default_redact_probe_port")]
    pub redact_probe_port: u16,

    /// Whether LLM caching is enabled (default: false)
    #[serde(default = "default_llm_cache_enabled")]
    #[serde(deserialize_with = "de_bool")]
    pub llm_cache: bool,

    /// emulate work and sleep on cached response
    #[serde(default = "default_llm_cache_sleep_millis")]
    pub llm_cache_sleep_millis: u32,
}

/// default value for work emulation sleep on cached response
pub fn default_llm_cache_sleep_millis() -> u32 {
    0
}

/// Deserialize a boolean from various string representations
///
/// # Arguments
/// * `deserializer` - The Serde deserializer
///
/// # Returns
/// `Result<bool, D::Error>` - Parsed boolean or error
///
/// # Supported formats
/// True values: "true", "1", "yes", "y"
/// False values: "false", "0", "no", "n"
fn de_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    match s.to_lowercase().as_str() {
        "true" | "1" | "yes" | "y" => Ok(true),
        "false" | "0" | "no" | "n" => Ok(false),
        _ => Err(serde::de::Error::custom(format!("Invalid boolean: {}", s))),
    }
}

/// Default LLM cache setting (false)
fn default_llm_cache_enabled() -> bool {
    false
}

/// Default redaction probe port (8118)
fn default_redact_probe_port() -> u16 {
    8118
}

/// Default LLM service URL ("http://localhost:4000/chat/completions")
fn default_llm_url() -> String {
    "http://localhost:4000/chat/completions".to_string()
}

/// Default maximum parallel redaction tasks (8)
fn default_redact_max_tasks() -> usize {
    8
}

/// Default system prompt location ("~/.local/system_prompt.txt")
fn default_system_prompt_location() -> String {
    expand_user_path("~/.local/system_prompt.txt")
        .to_string_lossy()
        .into_owned()
}

/// Default AWS region for S3 ("us-east-1")
fn default_aws_region_s3() -> String {
    "us-east-1".to_string()
}

/// Default queue stream max age (86400 seconds = 1 day)
fn default_queue_stream_max_age() -> u64 {
    60 * 60 * 24
}

/// Default LLM token ("sk-1234")
fn default_llm_token() -> SecretString {
    SecretString::new("sk-1234")
}

/// Default NATS URL ("nats://localhost:4222")
fn default_nats_url() -> String {
    "nats://localhost:4222".to_string()
}

/// Default log level ("info")
fn default_log_level() -> String {
    "info".to_string()
}

/// Default redaction subject ("redact-log")
fn default_redact_subject() -> String {
    "redact-log".to_string()
}

/// Default queue stream name ("converastion-stream")
fn default_queue_stream() -> String {
    "converastion-stream".to_string()
}

impl Cfg {
    /// Load configuration from environment variables
    ///
    /// # Panics
    /// Panics if required environment variables are missing or invalid
    pub fn from_env() -> Self {
        envy::from_env::<Cfg>().unwrap_or_else(|err| {
            panic!("Failed to load configuration : {}", err);
        })
    }

    /// Pretty print the current configuration
    ///
    /// Logs all configuration values at INFO level, with sensitive fields redacted
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
