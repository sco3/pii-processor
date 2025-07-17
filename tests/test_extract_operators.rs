use ductaper::llm_work::masks::get_valid_redactions;
use ductaper::util::logging::init_tracing;
use std::fs;
use tracing::info;

#[test]

pub fn test_extract_operators() {
    init_tracing();
    let prompt = fs::read_to_string("data/system_prompt.txt").unwrap();
    match get_valid_redactions(prompt.as_str()) {
        Some(ops) => {
            info!("Operators: {:?}", ops);
            assert_eq!(ops.len(), 16);

            for s in ops {
                assert!(s.starts_with('['));
                assert!(s.ends_with(']'));
            }
        }
        None => {
            panic!("No redactions found")
        }
    }
}
