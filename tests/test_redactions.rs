use crate::common::dummy_saver::DummySaver;
use common::dummy_caller::DummyCaller;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use serde_json::Value;
use std::fs::read_to_string;
use std::sync::Arc;

mod common;
#[test]
pub fn test_redactions() -> Result<(), Box<dyn std::error::Error>> {
    let response = read_to_string("tests/data/response.json")?;
    let v = serde_json::from_str::<Value>(&response)?;
    let content = LlmLogProcessor::extract_content(&v).unwrap();

    let proc = LlmLogProcessor::new(
        Arc::new(DummyCaller {}), //
        r#" 
            OPERATORS = {
                "DEFAULT": {"type": "replace", "new_value": "[REDACTED]"},
                "PERSON": {"type": "replace", "new_value": "[PERSON]"}
            }
        "#
        .to_string(),
        "nova".to_string(),
        Arc::new(DummySaver::new()),
    );
    let vr = &proc.valid_redactions.clone();
    println!("vr: {:?}", vr);
    let r = proc.parse_redactions(content).unwrap();
    println!("Redactions: {:?}", r);
    assert_eq!(r.len(), 3);

    Ok(())
}
