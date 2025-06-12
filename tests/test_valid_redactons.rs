use ductaper::llm_work::extract_operators::get_valid_redactions;

#[test]
pub fn test_read_valid_redactions() {
    let prompt = std::fs::read_to_string(
        "tests/data/example_system_prompt.txt", //
    )
    .unwrap();

    let redactions = get_valid_redactions(prompt.as_str());
    if let Some(redactions) = redactions {
    } else {
        panic!("Failed to get valid redactions!");
    }
}
