use std::fs::read_to_string;

pub fn read_prompt(location: &str) -> String {
    read_to_string(location).unwrap_or_else(|e| {
        panic!(
            "Failed to read system prompt: {} {}", //
            e, location
        )
    })
}
