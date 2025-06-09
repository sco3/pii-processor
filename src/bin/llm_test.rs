use ductaper::init_logging::init_tracing;
use ductaper::llm_caller::LLmCaller;
use ductaper::reducter::ReDucter;
use std::env::var;
use std::fs::read_to_string;
use std::time::Instant;
use tracing::info;

const URL: &str = "http://0.0.0.0:4000/chat/completions";
const TOKEN: &str = "sk-1234";

#[tokio::main]
async fn main() {
    init_tracing();

    let models = vec![/*"nova",*/ "haiku"];

    let system_prompt = read_to_string("data/system_prompt.txt") //
        .unwrap();
    let session_log = read_to_string("tests/data/worker-pool-test.json").unwrap();

    if var("ASDF").unwrap_or_default() == "ASDF" {
        simple_tests(&models, &system_prompt).await;
    }

    for model in &models {
        call_with_model(
            model, //
            system_prompt.as_str(),
            session_log.as_str(),
        )
        .await;
    }
}

async fn simple_tests(models: &Vec<&str>, system_prompt: &String) {
    for model in models {
        call_with_model(model, "Hello", "Hi").await;
    }

    for model in models {
        call_with_model(
            model, //
            r#"
        You are a PII redactor that only returns redacted text
        with no additional text or explanations.
        "#,
            "Hello I am Jack Daniels.",
        )
        .await;
    }

    for model in models {
        call_with_model(
            model, //
            system_prompt.as_str(),
            "Hello I am Jack Daniels.",
        )
        .await;
    }
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
    response_details(caller, model, prompt, msg).await;
    info!("Took: {} ms", start.elapsed().as_millis())
}

async fn response_details(caller: LLmCaller, model: &str, prompt: &str, msg: &str) {
    match caller.call(model, prompt, msg).await {
        Some(v) => {
            info!("Model: {}", v["model"]);
            info!("----");
            info!("Call result: {:?}", v);
            let choices = &v["choices"];
            info!("Choices: {:?}", choices);

            if let Some(choice_array) = choices.as_array() {
                for choice in choice_array {
                    info!("Choice {:?}", choice);
                    let msg = choice["message"]["content"].clone();
                    info!("----");
                    info!("\n\nMessage {}\n\n", msg);
                    info!("----")

                    // msg[""]
                }
            }
        }

        None => {
            info!("No value")
        }
    }
}
