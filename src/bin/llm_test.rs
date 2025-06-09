use ductaper::init_logging::init_tracing;
use ductaper::llm_caller::LLmCaller;
use ductaper::reducter::ReDucter;
use tracing::info;

#[tokio::main]
async fn main() {
    init_tracing();
    call_hi_with_model("haiku").await;
    call_hi_with_model("nova").await;
}

async fn call_hi_with_model(model: &str) {
    let url = "http://0.0.0.0:4000/chat/completions".to_string();
    let caller = LLmCaller::new(
        url, //
        model.to_string(),
        Some("sk-1234".to_string()),
    );
    info!("Model: {} --------------------- ", model);
    match caller.call("Hello", "Hi").await {
        Some(v) => {
            info!("Model: {}", v["model"]);
            info!("----");
            info!("Call result: {:?}", v);
            let choices = &v["choices"];
            info!("Choices: {:?}", choices);
            if choices.is_array() {
                if let Some(choice_array) = choices.as_array() {
                    for choice in choice_array {
                        info!("Choice {:?}", choice);
                        info!("Message {}", choice["message"]["content"]);
                        info!("----")
                    }
                }
            }
        }

        None => {
            info!("No value")
        }
    }
}
