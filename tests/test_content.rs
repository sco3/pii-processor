use redact::llm_work::llm_log_processor::LlmLogProcessor;
use redact::util::logging::init_tracing;
use serde_json::json;

#[test]

pub fn test_content() {
    init_tracing();
    let v = json!({
            "choices":[
                {"message": {"content":"-content-"}}
            ]
        }
    );

    let content = &v["choices"][0]["message"]["content"];
    if let Some(s) = content.as_str() {
        println!("{s}");
    }
    println!("Content: {content}");
    assert_eq!("-content-", content);
    {
        let cnt = LlmLogProcessor::extract_content(&v);
        assert_eq!(Some("-content-"), cnt);
    }
}

#[test]
pub fn test_content_invalid() {
    init_tracing();
    let v = json!({
            "choices":[
                {"message": {}}
            ]
        }
    );

    let content = &v["choices"][0]["message"]["content"];
    if let Some(s) = content.as_str() {
        println!("String: {s}");
        panic!("How come?");
    } else {
        println!("Not string: {content}");
    }
    {
        let cnt = LlmLogProcessor::extract_content(&v);
        assert_eq!(None, cnt);
    }
    {
        let cnt = LlmLogProcessor::extract_content(&v);
        assert_eq!(None, cnt);
    }
}
