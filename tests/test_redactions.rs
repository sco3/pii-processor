use crate::common::dummy_saver::DummySaver;
use common::dummy_caller::DummyCaller;
use redact::llm_work::llm_log_processor::LlmLogProcessor;
use serde_json::Value;
use std::fs::read_to_string;
use std::sync::Arc;

mod common;
#[test]
pub fn test_redactions() -> Result<(), Box<dyn std::error::Error>> {
    let response = read_to_string("tests/data/response.json")?;
    let v = serde_json::from_str::<Value>(&response)?;
    let content = LlmLogProcessor::extract_content(&v).unwrap();
    // prompt should include OPERATORS followed by empty line:
    let test_prompt = r#"
            OPERATORS = {
                "DEFAULT": {"type": "replace", "new_value": "[REDACTED]"},
                "PERSON": {"type": "replace", "new_value": "[PERSON]"}
            }

        "#
    .to_string();

    let proc = LlmLogProcessor::new(
        Arc::new(DummyCaller::new(None)), //
        test_prompt,
        "nova",
        Arc::new(DummySaver::new()),
    );
    let vr = &proc.valid_redactions.clone();
    println!("vr: {vr:?}");
    assert!(!vr.is_none());

    println!("Content: {}", content);
    {
        let r = proc.parse_redactions(content).unwrap();
        println!("Redactions: {r:?}");
        assert_eq!(r.len(), 3);
    }
    {
        let r = proc.redactions(&v);
        println!("Redaction2: {:?}", r);
        assert_eq!(r.len(), 3);
    }
    // invalid redaction is [ASDF] - should be only 2
    {
        let reversed = r#"
        {
            "redactions": {
                "asdf":"[ASDF]",
                "user@example.com": "user@e**********",
                "1234 5678 1234 5678": "1234 **** **** ****"
            }
        }
        "#;
        let r = proc.parse_redactions(reversed);
        if let Some(r) = r {
            println!("Redactions: {r:?}");
            assert_eq!(r.len(), 2);
        } else {
            panic!("Wrong response: {:?}", r);
        }
    }
    Ok(())
}

#[test]
pub fn test_redactions_without_valid_redactions() -> Result<(), Box<dyn std::error::Error>> {
    // prompt should include OPERATORS, but if not found all should work as well.
    // the empty line is missing after OPERATORS
    let test_prompt = r#"OPERATORS = {
                "DEFAULT": {"type": "replace", "new_value": "[REDACTED]"},
                "PERSON": {"type": "replace", "new_value": "[PERSON]"}
            }"#
    .to_string();

    let proc = LlmLogProcessor::new(
        Arc::new(DummyCaller::new(None)), //
        test_prompt,
        "nova",
        Arc::new(DummySaver::new()),
    );

    {
        let reversed = r#"
        {
            "redactions": {
                "asdf":"[ASDF]",
                "user@example.com": "user@e**********",
                "1234 5678 1234 5678": "1234 **** **** ****"
            }
        }
        "#;
        // no valid redactions
        let vr = &proc.valid_redactions.clone();
        println!("vr: {vr:?}");
        assert!(vr.is_none());

        let r = proc.parse_redactions(reversed);
        if let Some(r) = r {
            println!("Redactions: {r:?}");
            assert_eq!(r.len(), 3);
        } else {
            panic!("Wrong response: {:?}", r);
        }
    }

    Ok(())
}
