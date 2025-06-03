use crate::ai_tags::Ai;
use mime::APPLICATION_JSON;
use reqwest::RequestBuilder;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use serde_json::json;
use tracing::debug;
use tracing::error;

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
        let bearer = Self::add_bearer(token);

        LLmCaller {
            endpoint: endpoint.into(),
            model: model.into(),
            bearer,
            client: reqwest::Client::new(),
        }
    }

    fn add_bearer(token: Option<String>) -> Option<String> {
        token
            .as_ref() //
            .map(|t| format!("{} {}", Ai::BEARER, t))
    }

    fn build_body(&self, prompt: &str, message: &str) -> Value {
        json!({
            Ai::MODEL: self.model,
            Ai::MESSAGES: [
                { Ai::ROLE: Ai::SYSTEM, Ai::CONTENT: prompt },
                { Ai::ROLE: Ai::USER, Ai::CONTENT: message }
            ],
            Ai::TEMPERATURE: 0
        })
    }

    fn build_request(&self, body: Value) -> RequestBuilder {
        let mut req = self
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .body(body.to_string());

        if let Some(ref bearer) = self.bearer {
            req = req.header(AUTHORIZATION, bearer);
        }
        req
    }

    pub async fn send(req: RequestBuilder) -> Option<Value> {
        match req.send().await {
            Ok(res) => match res.json::<Value>().await {
                Ok(body) => {
                    debug!("Response: {}", body);
                    Some(body)
                }
                Err(e) => {
                    error!("Failed to parse JSON response: {}", e);
                    None
                }
            },
            Err(e) => {
                error!("Request failed: {}", e);
                None
            }
        }
    }
    pub async fn call(&self, prompt: &str, message: &str) -> Option<Value> {
        let body = self.build_body(prompt, message);
        let req = self.build_request(body);
        Self::send(req).await
    }
}
