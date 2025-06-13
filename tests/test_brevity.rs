use ductaper::llm_work::brevity::brevify;
use ductaper::logging::init_tracing;
use tracing::info;

#[test]
pub fn test_brevity() -> Result<(), Box<dyn std::error::Error>> {
    init_tracing();
    let content = std::fs::read_to_string(
        "tests/data/example_system_prompt.txt", //
    )?;
    let out = brevify(content.as_str());
    info!("Briefly?: {:?}", out);
    Ok(())
}
