use ductaper::llm_caller::LLmCaller;
use ductaper::logging;
use ductaper::logging::init_log;
use httpmock::Method::POST;
use httpmock::MockServer;
use serde_json::json;
use tracing_subscriber::FmtSubscriber;

pub fn init_tracing() {
    logging::LOG_INIT.call_once(|| {
        let subscriber = FmtSubscriber::builder()
            .with_max_level(tracing::Level::DEBUG)
            .with_test_writer()
            .finish();

        if let Err(e) = tracing::subscriber::set_global_default(subscriber) {
            println!("Sorry. Tracing already initialized: {}", e);
        }
    });
}
#[tokio::test]
async fn test_llm_caller() {
    init_log(&"debug".to_string());
    let server = MockServer::start();

    let expected_body = json!(
    {"choices":[
        {"finish_reason":"stop","index":0,
         "message":{"content":"MOCK: Hello!",
         "function_call":null,
         "role":"assistant","tool_calls":null}}
        ],
        "created":1748970411,
        "id":"chatcmpl-7c71612e-32dd-42",
        "model":"amazon.nova-lite-v1:0",
        "object":"chat.completion",
        "system_fingerprint":null,
        "usage":{
            "cache_creation_input_tokens":0,
            "cache_read_input_tokens":0,
            "completion_tokens":42,
            "completion_tokens_details":null,
            "prompt_tokens":42,
            "prompt_tokens_details":{
                "audio_tokens":null,"cached_tokens":0
            },
            "total_tokens":42
        }
    });

    let expected_str = serde_json::to_string(&expected_body) //
        .unwrap_or_default();

    let mock = server.mock(|when, then| {
        when.method(POST).path("/chat/completions");
        then.status(200).body(expected_str);
    });

    // proxy url sample
    // "http://0.0.0.0:4000/chat/completions".to_string(),

    let caller = LLmCaller::new(
        server.url("/chat/completions").to_string(),
        "nova".to_string(),
        Some("sk-1234".to_string()),
    );
    caller.call("Hello", "Hi").await;
    mock.assert();
}

#[tokio::test]
async fn test_send_request_failure() {
    init_tracing();
    let server = MockServer::start();

    // Simulate 500 Internal Server Error
    let _mock = server.mock(|when, then| {
        when.method(POST);
        then.status(500).body("Internal Server Error");
    });

    let caller = LLmCaller::new(server.url("/fail"), "gpt-test", None);
    let req = caller.build_request(json!({"key": "value"}));

    let result = LLmCaller::send(req).await;
    assert!(result.is_none(), "Expected None on HTTP failure");
}

#[tokio::test]
async fn test_send_json_parse_failure() {
    let server = MockServer::start();

    // Simulate valid HTTP 200 with invalid JSON body
    let _mock = server.mock(|when, then| {
        when.method(POST);
        then.status(200)
            .header("Content-Type", "application/json")
            .body("not-json");
    });

    let caller = LLmCaller::new(server.url("/bad-json"), "gpt-test", None);
    let req = caller.build_request(json!({"key": "value"}));

    let result = LLmCaller::send(req).await;
    assert!(result.is_none(), "Expected None on JSON parse failure");
}

#[tokio::test]
async fn test_no_server_failure() {
    let caller = LLmCaller::new("http://127.0.0.1:1", "gpt-test", None);
    let req = caller.build_request(json!({"key": "value"}));

    let result = LLmCaller::send(req).await;
    assert!(result.is_none(), "Expected None - no server");
}
