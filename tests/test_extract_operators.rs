use ductaper::llm_work::extract_operators::extract_and_parse_operators_fragment;
use ductaper::logging::init_tracing;
use std::fs;
use tracing::info;

#[test]

pub fn test_extract_operators() {
    init_tracing();
    let prompt = fs::read_to_string("data/system_prompt.txt").unwrap();
    match extract_and_parse_operators_fragment(prompt.as_str()) {
        Ok(ops) => {
            info!("Operators: {:?}", ops)
        }
        Err(e) => {
            panic!("Error: {}", e)
        }
    }
}
