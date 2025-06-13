use std::env;
use tracing::info;

pub fn list_env() {
    for (key, value) in env::vars() {
        if key.starts_with("AWS_") {
            info!("{key}: {}", mask(value));
        } else {
            info!("{key}: {value}");
        }
    }
}

pub fn mask(value: String) -> String {
    if value.len() > 4 {
        format!("{}****", &value[..4])
    } else {
        format!("{}****", value)
    }
}
