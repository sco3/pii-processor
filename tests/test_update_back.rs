mod common;
use common::dummy_caller::DummyCaller;
use common::dummy_saver::DummySaver;
use ductaper::llm_work::llm_log_processor::LlmLogProcessor;
use ductaper::logging::init_tracing;
use ductaper::session_log_models::SessionLog;
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tracing::info;

#[tokio::test]
pub async fn test_update_pii_redactions() {
    init_tracing();
    let mut reds = HashMap::new();
    reds.insert("Joulie Yen".to_string(), "[PERSON]".to_string());
    reds.insert("1234 5678 1235 1236".to_string(), "[CARD]".to_string());

    let data = fs::read_to_string("tests/data/to_update.json").unwrap();
    let mut log = serde_json::from_str::<SessionLog>(data.as_str()).unwrap();
    let proc = LlmLogProcessor::new(
        Arc::new(DummyCaller {}), //
        "".to_string(),
        "".to_string(),
        Arc::new(DummySaver::new()),
    );
    proc.update_log(&mut log, &reds);

    let out_str = serde_json::to_string_pretty(&log).unwrap();
    info!("Out: {}", out_str);
    for orig in reds.keys() {
        assert!(!out_str.contains(orig));
    }
}
