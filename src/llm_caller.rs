use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;
use tracing::debug;

const BEARER: &str = "Bearer";

pub struct LLmCaller {
    pub endpoint: String,
    pub model: String,
    pub token: Option<String>,
    pub client: reqwest::Client,
    pub bearer: Option<String>,
}

impl LLmCaller {
    pub fn new(endpoint: String, model: String, token: Option<String>) -> Self {
        let client = reqwest::Client::new();
        let bearer = token
            .as_ref() //
            .map(|t| format!("{} {}", BEARER, t));

        LLmCaller {
            endpoint,
            model,
            token,
            client,
            bearer,
        }
    }

    pub async fn call(&self, prompt: &str, message: &str) {
        let mut body = json!({
            "model": json!(self.model),
            "messages": [
                { "role": "system", "content": ""},
                { "role": "user", "content": "" }
            ],
            "temperature":0
        });

        body["messages"][0]["content"] = json!(prompt);
        body["messages"][1]["content"] = json!(message);

        let str_body = serde_json::to_string(&body) //
            .unwrap_or("{}".to_string());

        let mut req = self //
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json");

        if let Some(ref bearer) = self.bearer {
            req = req.header(AUTHORIZATION, bearer);
        }
        if let Ok(res) = req.body(str_body).send().await {
            let body = res //
                .json::<serde_json::Value>()
                .await
                .unwrap_or(json!({}));

            let resp_body = serde_json::to_string(&body) //
                .unwrap_or_default();

            debug!("Response: {}", resp_body);
        }
    }
}
