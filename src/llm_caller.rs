use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use serde_json::json;
use tracing::debug;
use tracing::error;

const BEARER: &str = "Bearer";

pub struct LLmCaller {
    pub endpoint: String,
    pub model: String,
    pub bearer: Option<String>,
    pub client: reqwest::Client,
}

impl LLmCaller {
    pub fn new(
        endpoint: impl Into<String>,
        model: impl Into<String>,
        token: Option<String>,
    ) -> Self {
        let bearer = token.as_ref().map(|t| format!("{} {}", BEARER, t));
        LLmCaller {
            endpoint: endpoint.into(),
            model: model.into(),
            bearer,
            client: reqwest::Client::new(),
        }
    }

    pub async fn call(&self, prompt: &str, message: &str) -> Option<Value> {
        let body = json!({
            "model": self.model,
            "messages": [
                { "role": "system", "content": prompt },
                { "role": "user", "content": message }
            ],
            "temperature": 0
        });

        let req = self
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, "application/json")
            .body(body.to_string());

        let req = if let Some(ref bearer) = self.bearer {
            req.header(AUTHORIZATION, bearer)
        } else {
            req
        };

        match req.send().await {
            Ok(res) => {
                let body = res.json::<Value>().await.unwrap_or_else(|e| {
                    debug!("Failed to parse JSON: {}", e);
                    json!({})
                });
                debug!("Response: {}", body);
                Some(body)
            }
            Err(e) => {
                error!("Request failed: {}", e);
                None
            }
        }
    }
}
