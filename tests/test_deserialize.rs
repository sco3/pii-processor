use ductaper::data::session_log_models::{SessionLog, SessionLogEntry};

use std::fs;

#[test]
fn test_deserialize_example_new_fields() {
    // Load test file from relative path
    let path = "tests/data/example_new_fields.json";
    let file_content = fs::read_to_string(path) //
        .expect("Failed to read example_new_fields.log");

    // Attempt to deserialize JSON into SessionLogType (Vec<SessionLogEntry>)
    let session_log: SessionLog = serde_json::from_str(&file_content) //
        .expect("Failed to deserialize session log");

    // Basic sanity checks - adjust depending on expected content
    assert!(!session_log.is_empty(), "Session log should not be empty");

    // For example, check first entry type
    match &session_log[0] {
        SessionLogEntry::ArchTypeEnum(msg) => {
            let str = serde_json::to_string(msg).unwrap_or_default();
            assert_eq!(str, r#"{"architecture_type":"Neocortex"}"#);
        }
        other => panic!(
            "Expected first entry to be ArchType, got {:?}", //
            other
        ),
    }

    assert_eq!(session_log.len(), 23, "Expected 23 entries in session log");
    let out = serde_json::to_string_pretty(&session_log).unwrap();
    //fs::write("/tmp/test_deserialize.rs", out).unwrap();
    let diff = (out.len() as f64 - file_content.len() as f64).abs();
    assert!(diff <= 10.0);
}
