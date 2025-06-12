use regex::Regex;
use serde_json::Value;
use tracing::debug;

/// Extracts a JSON-like fragment (specifically, a Rust HashMap or Python dictionary
/// literal) from a given text string and parses it into a serde_json::Value.
///
/// This function looks for a pattern that starts with "OPERATORS = {" and ends
/// with the corresponding closing "}". It then cleans up the extracted fragment
/// to make it valid JSON before parsing.
///
/// Arguments:
/// * `text`: The input string containing the fragment.
///
/// Returns:
/// * `Result<Value, String>`: Ok(Value) if parsing is successful,
///                            Err(String) if the fragment cannot be found or parsed.
pub fn extract_and_parse_operators_fragment(text: &str) -> Result<Value, String> {
    // Regex to find the "OPERATORS = {" and capture everything up to and including the closing "}".
    // The capturing group `([\s\S]*?\n\})` includes the outer curly braces.
    let re = Regex::new(r"OPERATORS\s*=\s*(\{[\s\S]*?\n\})")
        .map_err(|e| format!("Failed to compile regex: {}", e))?;

    // Find the match
    let captures = re
        .captures(text)
        .ok_or_else(|| "OPERATORS fragment not found in the text.".to_string())?;

    // Extract the captured group, which is the entire dictionary/map content including its outer braces.
    let raw_fragment_with_braces = captures
        .get(1)
        .ok_or_else(|| "Failed to capture the OPERATORS content.".to_string())?
        .as_str();

    debug!("Raw fragment: {}", raw_fragment_with_braces);

    // The extracted fragment might contain trailing commas, which are allowed in Python/Rust
    // literals but not standard JSON. We need to remove them.
    let mut clean_json_string = raw_fragment_with_braces.to_string();

    // Regex to remove trailing commas before a closing brace or end of string.
    // It captures the brace or end of string and replaces the comma and whitespace with just that.
    let trailing_comma_re = Regex::new(r",\s*(\}|$)")
        .map_err(|e| format!("Failed to compile trailing comma regex: {}", e))?;
    clean_json_string = trailing_comma_re
        .replace_all(&clean_json_string, "$1")
        .to_string();

    // At this point, `clean_json_string` already contains the valid JSON object
    // (e.g., `{"key": "value"}`). It does not need further wrapping.

    // Debugging print
    // println!("Cleaned JSON fragment for parsing:\n{}", clean_json_string);

    // Parse the cleaned JSON string
    serde_json::from_str(&clean_json_string)
        .map_err(|e| format!("Failed to parse JSON fragment: {}", e))
}
