use redact::llm_work::llm_log_processor::LlmLogProcessor;
use redact::util::logging::init_tracing;

#[test]
pub fn test_parse() {
    init_tracing();
    let buf = b"";
    let log = LlmLogProcessor::parse(buf);
    assert!(log.is_none());
}
