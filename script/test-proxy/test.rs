use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use std::{fs, time::Instant};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    let system_prompt = fs::read_to_string("system_prompt.txt")?;

    let mut data: serde_json::Value = serde_json::from_str(&fs::read_to_string("input.json")?)?;
    data["messages"][0]["content"] = json!(system_prompt);

    let json_body = serde_json::to_string(&data)?;

    let start = Instant::now();
    let resp = client
        .post("http://0.0.0.0:4000/chat/completions")
        .header(AUTHORIZATION, "Bearer sk-1234")
        .header(CONTENT_TYPE, "application/json")
        .body(json_body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    println!("{}", resp);
    println!("Took: {} ms", start.elapsed().as_millis());

    if let Some(content) = resp["choices"][0]["message"]["content"].as_str() {
        println!("{}", content);
    } else {
        println!("Not found.");
    }

    Ok(())
}
