use std::env;
use tracing::info;

/// Logs all environment variables, masking AWS credentials.
pub fn list_env() {
    for (key, value) in env::vars() {
        if key.starts_with("AWS_") {
            info!("{key}: {}", mask(&value));
        } else {
            info!("{key}: {value}");
        }
    }
}

/// Masks sensitive strings by showing first 4 chars followed by '****'.
#[must_use]
pub fn mask(value: &str) -> String {
    if value.len() > 4 {
        format!("{}****", &value[..4])
    } else {
        format!("{value}****")
    }
}
