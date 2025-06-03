use ductaper::llm_caller::LLmCaller;
#[test]
fn test_llm_caller() {
    let caller = LLmCaller::_new(
        "https://api.openai.com/v1/chat/completions".to_string(),
        "gpt-3.5-turbo".to_string(),
        Some("sk-1234".to_string()),
    );

    assert_eq!(
        caller.endpoint,
        "https://api.openai.com/v1/chat/completions"
    );
}
