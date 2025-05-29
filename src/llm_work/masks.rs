use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use tracing::debug;
/// sometimes LLM produces replacement which are not defined in ssytem prompt
/// parse system prompt to find valid masks
pub fn get_valid_redactions(text: &str) -> Option<HashSet<String>> {
    // Extract the JSON fragment first
    let json_fragment = extract_json_fragment(text)?;
    debug!("Prompt defines following redactions: {}", json_fragment);

    // Clean the JSON string
    let clean_json = remove_trailing_commas(&json_fragment)?;

    // Parse and process the JSON
    parse_and_extract_redactions(&clean_json)
}
///
/// system prompt includes masks:
///
/// OPERATORS = {
/// "DEFAULT": {"type": "replace", "`new_value"`: "[REDACTED]"},
/// "PERSON": {"type": "replace", "`new_value"`: "[PERSON]"},
/// "`PHONE_NUMBER"`: {"type": "mask", "`masking_char"`: "*", "`chars_to_mask"`: 6, "`from_end"`: true},
///
/// find fragment with replacements by finding "OPERATORS" (see also `data/system_prompt.txt`
fn extract_json_fragment(text: &str) -> Option<String> {
    let re = Regex::new(r"\s*OPERATORS\s*=\s*(\{[\s\S]*\}\s*\n\s*\n)").ok()?;

    let captures = re.captures(text)?;
    Some(captures.get(1)?.as_str().to_string())
}

/// removing trailing command from extracted fragment
fn remove_trailing_commas(json: &str) -> Option<String> {
    let re = Regex::new(r",\s*(}|$)").ok()?;
    Some(re.replace_all(json, "$1").to_string())
}

/// parse system prompt and check for masks
fn parse_and_extract_redactions(json: &str) -> Option<HashSet<String>> {
    let redactions = serde_json::from_str::<Value>(json).ok()?;

    let map = redactions.as_object()?;
    let masks: HashSet<String> = map
        .values()
        .filter_map(|v| {
            v["new_value"]
                .as_str() //
                .map(String::from)
        })
        .collect();
    for mask in &masks {
        debug!("Prompt defines mask: {}", mask);
    }

    (!masks.is_empty()).then_some(masks)
}
