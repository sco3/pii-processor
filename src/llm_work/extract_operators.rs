use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;
use tracing::debug;

pub fn get_valid_redactions(text: &str) -> Option<HashSet<String>> {
    // Extract the JSON fragment first
    let json_fragment = extract_json_fragment(text)?;
    debug!("Raw fragment: {}", json_fragment);

    // Clean the JSON string
    let clean_json = remove_trailing_commas(&json_fragment)?;

    // Parse and process the JSON
    parse_and_extract_redactions(&clean_json)
}

fn extract_json_fragment(text: &str) -> Option<String> {
    let re = Regex::new(r"OPERATORS\s*=\s*(\{[\s\S]*?\n})").ok()?;
    let captures = re.captures(text)?;
    Some(captures.get(1)?.as_str().to_string())
}

fn remove_trailing_commas(json: &str) -> Option<String> {
    let re = Regex::new(r",\s*(}|$)").ok()?;
    Some(re.replace_all(json, "$1").to_string())
}

fn parse_and_extract_redactions(json: &str) -> Option<HashSet<String>> {
    let redactions = serde_json::from_str::<Value>(json).ok()?;
    debug!("Redactions: {}", redactions);

    let map = redactions.as_object()?;
    let valid_replacements: HashSet<String> = map
        .values()
        .filter_map(|v| {
            v["new_value"]
                .as_str() //
                .map(String::from)
        })
        .collect();

    (!valid_replacements.is_empty()).then_some(valid_replacements)
}
