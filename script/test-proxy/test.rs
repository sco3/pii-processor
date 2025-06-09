use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::fs;
use std::time::Instant;

//use tokio;

#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();

    let system_prompt = fs::read_to_string("system_prompt.txt").unwrap_or_default();
    let message = fs::read_to_string("worker-pool-test.json").unwrap_or_default();
    let template = fs::read_to_string("input.json").unwrap_or_default();

    let mut json_body: serde_json::Value = serde_json::from_str(&template).unwrap_or(json!({}));
    json_body["messages"][0]["content"] = json!(system_prompt);
    json_body["messages"][1]["content"] = json!(message);

    let str_body = serde_json::to_string_pretty(&json_body).unwrap_or("{}".to_string());
    let _ = fs::write("body.json", &str_body);

    let start = Instant::now();
    if let Ok(resp) = client
        .post("http://0.0.0.0:4000/chat/completions")
        .header(AUTHORIZATION, "Bearer sk-1234")
        .header(CONTENT_TYPE, "application/json")
        .body(str_body)
        .send()
        .await
    {
        let llm_took = resp
            .headers()
            .get("x-litellm-response-duration-ms")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<f32>().ok())
            .unwrap_or(0.0);

        let body = resp.json::<serde_json::Value>().await.unwrap_or(json!({}));
        let took = start.elapsed().as_millis() as f32;
        let extra = took - llm_took;

        println!("{}", body);

        if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
            println!("{}", content);
        }

        println!(
            "Took: {} ms LLM took: {} ms Extra: {} ms",
            took, llm_took, extra
        );
    }
}
