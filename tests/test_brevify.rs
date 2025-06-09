use ductaper::init_logging::init_tracing;
use ductaper::llm_work::brevity::brevify;
use std::fs::read_to_string;

use serde_json::json;
use std::fs;
use std::time::Instant;
use tracing::info;
#[test]
fn test_brevify() {
    let start = Instant::now();
    init_tracing();

    let session_log =
        read_to_string("tests/data/worker-pool-test.txt").expect("Failed to read session log");

    let (dict, abbr) = brevify(session_log.as_str());

    let output = json!({
        "dictionary": dict,
        "text": abbr
    });
    let content = serde_json::to_string_pretty(&output).unwrap();
    fs::write("/tmp/worker-pool-test.bvt", &content).expect("Write failed");
    info!("Dictionary: {:?}", dict);
    info!("Abbreviated text:\n{}", abbr);
    info!("Took {}", start.elapsed().as_millis());
    info!("Combined JSON:\n{}", content);
}
