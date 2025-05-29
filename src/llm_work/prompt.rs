/// system prompt load
use crate::llm_work::preview::preview_str;
use crate::util::exit_codes::ExitCode;
use std::fs::read_to_string;
use std::process::exit;
use tracing::{error, info};

/// reads system prompt
/// # Panics
/// the prompt should be readable and app exits with panic if not.
pub fn read_prompt(location: &str, exit_on_error: bool) -> String {
    match read_to_string(location) {
        Ok(prompt) => {
            info!(
                "System prompt is {} bytes: {} ...",
                prompt.len(),
                preview_str(&prompt)
            );
            prompt
        }
        Err(e) => {
            error!(
                "Failed to read system prompt from : {} {}", //
                location, e
            );
            if exit_on_error {
                exit(ExitCode::PromptError.code());
            } else {
                panic!("{}", e);
            }
        }
    }
}
