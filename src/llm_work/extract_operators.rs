use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use tracing::debug;

/// Extracts a JSON fragment
///
/// This function looks for a pattern that starts with "OPERATORS = {" and ends
/// with the corresponding closing "}". It then cleans up the extracted fragment
/// to make it valid JSON before parsing.
/// * `Option<HashSet<String>>` if parsing is successful or None

pub fn get_valid_redactions(text: &str) -> Option<HashSet<String>> {
    // Regex to find the "OPERATORS = {" and capture everything up to and including the closing "}".
    // The capturing group `([\s\S]*?\n\})` includes the outer curly braces.
    if let Ok(re) = Regex::new(r"OPERATORS\s*=\s*(\{[\s\S]*?\n\})") {
        // Find the match
        if let Some(captures) = re.captures(text) {
            // Extract the captured group, which is the entire dictionary/map content including its outer braces.
            if let Some(m) = captures.get(1) {
                let raw_fragment_with_braces = m.as_str();

                debug!("Raw fragment: {}", raw_fragment_with_braces);

                // The extracted fragment might contain trailing commas, which are allowed in Python/Rust
                // literals but not standard JSON. We need to remove them.
                let clean_json_string = raw_fragment_with_braces.to_string();

                // Regex to remove trailing commas before a closing brace or end of string.
                // It captures the brace or end of string and replaces the comma and whitespace with just that.
                if let Ok(trailing_comma_re) = Regex::new(r",\s*(\}|$)") {
                    let clean_json_string = trailing_comma_re
                        .replace_all(&clean_json_string, "$1")
                        .to_string();
                    // Parse the cleaned JSON string
                    if let Ok(redactions) = serde_json::from_str::<Value>(
                        &clean_json_string, //
                    ) {
                        debug!("Recactions: {}", redactions);
                        if let Some(map) = redactions.as_object() {
                            let mut valid_replacements: HashSet<String> = HashSet::new();
                            for (_, v) in map.iter() {
                                if let Some(s) = v["new_value"].as_str() {
                                    valid_replacements.insert(s.to_string());
                                }
                            }
                            if valid_replacements.len() > 0 {
                                return Some(valid_replacements);
                            }
                        }
                    }
                }
            }
        }
    }
    None
}
