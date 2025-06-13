use ductaper::llm_work::llm_caller::LLmCaller;
use ductaper::llm_work::pii_text::pii_text;
use ductaper::llm_work::reducter::ReDucter;
use ductaper::logging::init_tracing;
use ductaper::data::session_log_models::SessionLog;
use serde_json::Value;
use std::env::var;
use std::fs::read;
use std::fs::read_to_string;
use std::time::Instant;
use tracing::info;

const URL: &str = "http://0.0.0.0:4000/chat/completions";
const TOKEN: &str = "sk-1234";

#[tokio::main]
async fn main() {
    init_tracing();

    let models = vec!["haiku"];

    let system_prompt = read_to_string("data/system_prompt.txt") //
        .unwrap();

    // system_prompt.push_str("\n show only redactions dictionary.");

    let session_log = read("tests/data/worker-pool-test.json").unwrap();

    if var("ASDF").unwrap_or_default() == "ASDF" {
        simple_tests(&models, &system_prompt).await;
    }

    let session_log: SessionLog = serde_json::from_slice(session_log.as_ref()) //
        .expect("Failed to deserialize session log");

    let text = pii_text(&session_log);

    info!("Text: {}", text);
    let text2 = "[]";
    for model in &models {
        call_with_model(
            model, //
            system_prompt.as_str(),
            text2,
        )
        .await;
    }
}

async fn simple_tests(models: &Vec<&str>, system_prompt: &str) {
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
            system_prompt,
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
            let pretty_v = serde_json::to_string_pretty(&v).unwrap();
            println!("Call result: {}", pretty_v);
            let choices = &v["choices"];
            info!("Choices: {:?}", choices);

            if let Some(choice_array) = choices.as_array() {
                for choice in choice_array {
                    info!("Choice {:?}", choice);
                    let content = choice["message"]["content"].clone();
                    let content_str = content.as_str().unwrap_or_default();
                    info!("----");
                    match serde_json::from_str::<serde_json::Value>(content_str) {
                        Ok(parsed_json) => {
                            let pretty = serde_json::to_string_pretty(&parsed_json)
                                .unwrap_or_else(|_| content.to_string());

                            info!("\n\nParsed Message JSON:\n{}\n\n", pretty);
                            let redactions = &parsed_json["redactions"];
                            info!(
                                "\n\nRedactions:\n{}\n\n",
                                serde_json::to_string_pretty(redactions).unwrap()
                            );

                            if let Value::Object(map) = redactions {
                                for (key, value) in map {
                                    if let Some(s) = value.as_str() {
                                        info!("from: {} to {}", key, s);
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            // Not valid JSON, just print raw string
                            info!("\n\nMessage:\n{}\n\n", content_str);
                        }
                    }
                    info!("----")
                }
            }
        }
        None => {
            info!("No value")
        }
    }
}
