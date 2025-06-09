use ductaper::init_logging::init_tracing;
use ductaper::llm_caller::LLmCaller;
use ductaper::reducter::ReDucter;
use std::time::Instant;
use tracing::info;

const URL: &str = "http://0.0.0.0:4000/chat/completions";
const TOKEN: &str = "sk-1234";

#[tokio::main]
async fn main() {
    init_tracing();
    //call_with_model("haiku", "Hello", "Hi").await;
    //call_with_model("nova", "Hello", "Hi").await;
    call_with_model(
        "nova", //
        r#"
        You are a PII redactor that only returns redacted text 
        with no additional text or explanations.
        "#,
        "Hello I am Jack Daniels.",
    )
    .await;
}

async fn call_with_model(model: &str, prompt: &str, msg: &str) {
    let start = Instant::now();
    let url = URL.to_string();
    let caller = LLmCaller::new(
        url, //
        model.to_string(),
        Some(TOKEN.to_string()),
    );
    info!("Model: {} --------------------- ", model);
    response_details(caller, prompt, msg).await;
    info!("Took: {} ms", start.elapsed().as_millis())
}

async fn response_details(caller: LLmCaller, prompt: &str, msg: &str) {
    match caller.call(prompt, msg).await {
        Some(v) => {
            info!("Model: {}", v["model"]);
            info!("----");
            info!("Call result: {:?}", v);
            let choices = &v["choices"];
            info!("Choices: {:?}", choices);

            if let Some(choice_array) = choices.as_array() {
                for choice in choice_array {
                    info!("Choice {:?}", choice);
                    info!("Message {}", choice["message"]["content"]);
                    info!("----")
                }
            }
        }

        None => {
            info!("No value")
        }
    }
}
