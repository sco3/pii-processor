#![deny(missing_docs)]

//! Configuration module for Ductaper application.
//!
//! This module handles loading and managing configuration settings from environment variables,
//! with sensible defaults for various services including NATS, LLM, AWS, and logging.

use crate::config::env_vars_defaults::default_aws_region_s3;
use crate::config::env_vars_defaults::default_aws_s3_endpoint;
use crate::config::env_vars_defaults::default_llm_cache_enabled;
use crate::config::env_vars_defaults::default_llm_cache_sleep_millis;
use crate::config::env_vars_defaults::default_llm_token;
use crate::config::env_vars_defaults::default_llm_url;
use crate::config::env_vars_defaults::default_log_level;
use crate::config::env_vars_defaults::default_nats_url;
use crate::config::env_vars_defaults::default_queue_stream;
use crate::config::env_vars_defaults::default_queue_stream_max_age;
use crate::config::env_vars_defaults::default_redact_max_tasks;
use crate::config::env_vars_defaults::default_redact_probe_port;
use crate::config::env_vars_defaults::default_redact_subject;
use crate::config::env_vars_defaults::default_system_prompt_location;
use crate::config::env_vars_methods::de_bool;
use crate::config::secret_string::SecretString;
use serde::{Deserialize, Serialize};

/// Main configuration structure for the application.
///
/// Contains all configurable parameters with sensible defaults.
/// Most fields can be set via environment variables following the field names in uppercase.
#[derive(Deserialize, Debug, Clone, Serialize)]
pub struct Cfg {
    /// NATS server URL (default: "<nats://localhost:4222>")
    #[serde(default = "default_nats_url")]
    pub nats_url: String,

    /// Logging level (default: "info")
    #[serde(default = "default_log_level")]
    pub log_level: String,

    /// Token for LLM service (default: "sk-1234")
    #[serde(default = "default_llm_token")]
    pub llm_token: SecretString,

    /// URL for LLM service (default: "<http://localhost:4000/chat/completions>")
    #[serde(default = "default_llm_url")]
    pub llm_url: String,

    /// Tenant identifier (required)
    pub tenant: String,

    /// Application identifier (required)
    pub application: String,

    /// NATS stream name for queues (default: "conversation-stream")
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

    #[serde(default = "default_aws_s3_endpoint")]
    /// s3 endpoint for tests
    pub aws_s3_endpoint: Option<String>,

    /// Optional AWS access key ID
    pub aws_access_key_id: Option<SecretString>,

    /// Optional AWS secret access key
    pub aws_secret_access_key: Option<SecretString>,

    /// Optional AWS session token
    pub aws_access_token: Option<SecretString>,

    /// pass aws key as header: "x-portkey-aws-access-key-id"
    pub portkey_aws_access_key_id_header: Option<String>,

    /// pass aws region as header: "x-portkey-aws-region"
    pub portkey_aws_region_header: Option<String>,

    /// pass aws access key as header: "x-portkey-aws-secret-access-key"
    pub portkey_aws_secret_access_key_header: Option<String>,

    /// Optional AWS session token as header: "x-portkey-aws-session-token"
    pub portkey_aws_access_token_header: Option<String>,

    /// Optional portkey provider: "x-portkey-provider"
    pub portkey_provider_header: Option<String>,
    /// Optional portkey provider: "bedrock"
    pub portkey_provider_value: Option<String>,

    /// URL for aggregator sessions log
    pub aggregator_sessions_log_url: String,

    /// LLM model identifier
    pub llm_model: String,

    /// Location of system prompt file (default: "~/.`local/system_prompt.txt`")
    #[serde(default = "default_system_prompt_location")]
    pub system_prompt_location: String,

    /// Maximum parallel redaction tasks (default: 8)
    #[serde(default = "default_redact_max_tasks")]
    pub redact_max_tasks: usize,

    /// Redaction subject
    #[serde(default = "default_redact_probe_port")]
    pub redact_probe_port: u16,

    /// Whether LLM caching is enabled (default: false)
    #[serde(default = "default_llm_cache_enabled")]
    #[serde(deserialize_with = "de_bool")]
    pub llm_cache: bool,

    /// emulate work and sleep on cached response
    #[serde(default = "default_llm_cache_sleep_millis")]
    pub llm_cache_sleep_millis: u64,
    /// LLM region
    pub aws_region: Option<String>,
}
