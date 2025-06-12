use std::fs::read_to_string;
use tracing::error;

pub fn read_prompt(location: &String) -> String {
    read_to_string(location) //
        .unwrap_or_else(|e| {
            error!("Failed to read system prompt: {} {}", e, location);
            String::new()
        })
}
