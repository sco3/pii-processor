use ductaper::llm_caller::LLmCaller;
use ductaper::reducter::ReDucter;
use tracing::debug;
use ductaper::init_logging::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing();
    let url = "http://0.0.0.0:4000/chat/completions".to_string();
    let caller = LLmCaller::new(
        url, //
        "haiku".to_string(),
        Some("sk-1234".to_string()),
    );
    match caller.call("Hello", "Hi").await {
        Some(v) => {
            debug!("Call result: {:?}", v);
        }
        None => {
            debug!("No value")
        }
    }
}
