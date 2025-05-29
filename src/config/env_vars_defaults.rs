use crate::config::expanduser::expand_user_path;
use crate::config::secret_string::SecretString;

/// default value for work emulation sleep on cached response
#[must_use]
pub fn default_llm_cache_sleep_millis() -> u64 {
    0
}

/// Default LLM cache setting (false)
pub fn default_llm_cache_enabled() -> bool {
    false
}

/// Default redaction probe port (8118)
pub fn default_redact_probe_port() -> u16 {
    8118
}

/// Default maximum parallel redaction tasks (8)
pub fn default_redact_max_tasks() -> usize {
    8
}

/// Default system prompt location ("~/.`local/system_prompt.txt`")
pub fn default_system_prompt_location() -> String {
    expand_user_path("~/.local/system_prompt.txt")
        .to_string_lossy()
        .into_owned()
}

/// Default AWS region for S3 ("us-east-1")
pub fn default_aws_region_s3() -> String {
    "us-east-1".to_string()
}

/// Default AWS S3 endpoint (for tests)
pub fn default_aws_s3_endpoint() -> Option<String> {
    None
}

/// Default queue stream max age (86400 seconds = 1 day)
pub fn default_queue_stream_max_age() -> u64 {
    60 * 60 * 24
}

/// Default LLM token ("sk-1234")
pub fn default_llm_token() -> SecretString {
    SecretString::new("sk-1234")
}

/// Default NATS URL ("<nats://localhost:4222>")
pub fn default_nats_url() -> String {
    "nats://localhost:4222".to_string()
}

/// Default log level ("info")
pub fn default_log_level() -> String {
    "info".to_string()
}

/// Default redaction subject
pub fn default_redact_subject() -> String {
    "redact_log".to_string()
}

/// Default queue stream name ("conversation-stream")
pub fn default_queue_stream() -> String {
    "conversation-stream".to_string()
}
/// URL for LLM service (default: "<http://localhost:4000/chat/completions>")
pub fn default_llm_url() -> String {
    "http://localhost:4000/chat/completions".to_string() //
}
