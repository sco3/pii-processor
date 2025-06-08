use std::fs::read_to_string;
use tracing::error;

pub fn prompt(location: String) -> String {
    read_to_string(location) //
        .unwrap_or_else(|e| {
            error!("Failed to read system prompt: {}", e);
            String::new()
        })
}
