use crate::ai_tags::Ai;
use crate::reducter::ReDucter;

use async_trait::async_trait;
use mime::APPLICATION_JSON;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::RequestBuilder;
use serde_json::json;
use serde_json::Value;
use std::time::Instant;
use tracing::{debug, error, info};

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

    fn build_body(&self, model: &str, prompt: &str, message: &str) -> Value {
        json!({
            Ai::MODEL: model,
            Ai::MESSAGES: [
                { Ai::ROLE: Ai::SYSTEM, Ai::CONTENT: prompt },
                { Ai::ROLE: Ai::USER, Ai::CONTENT: message }
            ],
            Ai::TEMPERATURE: 0
        })
    }

    pub fn build_request(&self, body: Value) -> RequestBuilder {
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
        let res = match req.send().await {
            Ok(res) => res,
            Err(e) => {
                error!("Request failed: {:?}", e);
                return None;
            }
        };
        debug!("Call result status: {}", res.status());

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.ok();
            error!("Request failed with status {}: {:?}", status, text);
            return None;
        }

        match res.json::<Value>().await {
            Ok(body) => Some(body),
            Err(e) => {
                error!("Failed to parse JSON response: {}", e);
                None
            }
        }
    }
}
#[async_trait]
impl ReDucter for LLmCaller {
    async fn call(&self, model: &str, prompt: &str, message: &str) -> Option<Value> {
        let body = self.build_body(model, prompt, message);
        debug!("Request body: {}", pretty(&body));
        let req = self.build_request(body);
        let start = Instant::now();
        let output = Self::send(req).await;
        info!("Call took: {} ms", start.elapsed().as_millis());

        output
    }
}

fn pretty(body: &Value) -> String {
    serde_json::to_string_pretty(&body).unwrap_or_default()
}
