use dotenv::dotenv;
use ductaper::config::env_vars::Cfg;
use ductaper::data::session_log_models::SessionLog;
use ductaper::llm_work::get_text_from_session_log::get_text_from_session_log;
use ductaper::llm_work::llm_caller::LLmCaller;
use ductaper::llm_work::reducter::ReDucter;
use ductaper::util::logging::init_tracing;
use ductaper::worker_pool::serve::Stat;
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
    dotenv().ok();

    let cfg = Cfg::from_env();

    let models = vec!["haiku"];

    let system_prompt = read_to_string("data/system_prompt.txt") //
        .unwrap();

    // system_prompt.push_str("\n show only redactions dictionary.");

    let session_log = read("tests/data/worker-pool-test.json").unwrap();

    if var("ASDF").unwrap_or_default() == "ASDF" {
        simple_tests(&cfg, &models, &system_prompt).await;
    }

    let session_log: SessionLog = serde_json::from_slice(session_log.as_ref()) //
        .expect("Failed to deserialize session log");

    let text = get_text_from_session_log(&session_log);

    info!("Text: {}", text);
    let text2 = "[]";
    for model in &models {
        call_with_model(
            &cfg,
            model, //
            system_prompt.as_str(),
            text2,
        )
        .await;
    }
}

async fn simple_tests(cfg: &Cfg, models: &Vec<&str>, system_prompt: &str) {
    for model in models {
        call_with_model(cfg, model, "Hello", "Hi").await;
    }

    for model in models {
        call_with_model(
            cfg,
            model, //
            r"
        You are a PII redactor that only returns redacted text
        with no additional text or explanations.
        ",
            "Hello I am Jack Daniels.",
        )
        .await;
    }

    for model in models {
        call_with_model(
            cfg,
            model, //
            system_prompt,
            "Hello I am Jack Daniels.",
        )
        .await;
    }
}

async fn call_with_model(cfg: &Cfg, model: &str, prompt: &str, msg: &str) {
    let start = Instant::now();
    let url = URL;
    let caller = LLmCaller::new(
        url, //
        model,
        Some(&TOKEN.to_string()),
        false,
        0,
        cfg,
    );
    info!("Model: {} --------------------- ", model);
    response_details(caller, model, prompt, msg).await;
    info!("Took: {} ms", start.elapsed().as_millis());
}

async fn response_details(caller: LLmCaller, model: &str, prompt: &str, msg: &str) {
    let mut stat = Stat::default();
    match caller.call(model, prompt, msg, &mut stat).await {
        Some(v) => {
            info!("Model: {}", v["model"]);
            info!("----");
            let pretty_v = serde_json::to_string_pretty(&v).unwrap();
            println!("Call result: {pretty_v}");
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
                    info!("----");
                }
            }
        }
        None => {
            info!("No value");
        }
    }
}
