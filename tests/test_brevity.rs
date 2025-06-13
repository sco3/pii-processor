use ductaper::llm_work::brevity::brevify;

#[test]
pub fn test_brevity() -> Result<(), Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(
        "tests/data/example_system_prompt.txt", //
    )?;
    brevify(content.as_str());
    Ok(())
}
