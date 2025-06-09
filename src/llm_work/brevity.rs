use std::collections::HashMap;

/// Compresses repeated words in a text using a prefix-based abbreviation strategy.
///
/// Words that appear more than once are replaced with a short form (e.g., "question" -> "q1").
/// The abbreviation is based on the first letter and a running index.
/// Words starting with digits are not abbreviated.
///
/// Returns a dictionary mapping abbreviations back to their original words,
/// along with the compressed version of the text.
///
/// # Arguments
///
/// * `text` - Input text to compress.
///
/// # Returns
///
/// A tuple containing:
/// - `HashMap<String, String>`: A dictionary of abbreviations to original words.
/// - `String`: The compressed version of the input text.
pub fn brevify(text: &str) -> (HashMap<String, String>, String) {
    let mut dict = HashMap::new();
    let mut counts = HashMap::new();
    let mut id_counter: HashMap<char, usize> = HashMap::new();
    let mut word_to_abbr: HashMap<String, String> = HashMap::new();

    let words: Vec<String> = text
        .split_whitespace()
        .map(|w| w.trim_matches(|c: char| !c.is_alphanumeric()))
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .collect();

    // Count occurrences of each word
    for word in &words {
        *counts.entry(word.clone()).or_insert(0) += 1;
    }

    let mut result = Vec::new();

    for word in &words {
        let first_char = word.chars().next().unwrap_or('x');

        // If word starts with digit or appears only once, do not abbreviate
        if first_char.is_ascii_digit() || counts[word] <= 1 {
            result.push(word.clone());
            continue;
        }

        // If already abbreviated, reuse abbreviation
        if let Some(abbr) = word_to_abbr.get(word) {
            result.push(abbr.clone());
            continue;
        }

        // Create new abbreviation
        let id = id_counter.entry(first_char).or_insert(1);
        let abbr = format!("{}{}", first_char, *id);
        *id += 1;

        // Save mappings
        dict.insert(abbr.clone(), word.clone());
        word_to_abbr.insert(word.clone(), abbr.clone());

        result.push(abbr);
    }

    (dict, result.join(" "))
}
