use crate::data::ai_tags::Ai;
use crate::llm_work::reducter::ReDucter;

use async_trait::async_trait;
use mime::APPLICATION_JSON;
use moka::future::Cache;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use reqwest::RequestBuilder;
use serde_json::json;
use serde_json::Value;
use std::time::{Duration, Instant};
use tokio::time::sleep;
use tracing::{debug, error, info, Level};

pub struct LLmCaller {
    pub endpoint: String,
    pub model: String,
    pub bearer: Option<String>,
    pub client: reqwest::Client,
    pub cache_flag: bool,
    cache: Cache<Vec<u8>, Value>,
}

impl LLmCaller {
    pub fn new(
        endpoint: impl Into<String>,
        model: impl Into<String>,
        token: Option<String>,
        cache_flag: bool,
    ) -> Self {
        let bearer = Self::add_bearer(token);
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(3600))
            .max_capacity(1000)
            .build();

        LLmCaller {
            endpoint: endpoint.into(),
            model: model.into(),
            bearer,
            client: reqwest::Client::new(),
            cache_flag,
            cache,
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

    pub fn build_request(&self, body: &Value) -> RequestBuilder {
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

        let litellm_took = res
            .headers()
            .get("x-litellm-response-duration-ms")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("0");

        debug!("Litellm took: {} ms", litellm_took);

        match res.json::<Value>().await {
            Ok(body) => {
                if tracing::enabled!(Level::DEBUG) {
                    match serde_json::to_string_pretty(&body) {
                        Ok(pretty_body) => {
                            debug!("Response:\n{}", pretty_body);
                        }
                        Err(_) => {
                            debug!("Pretty print failed. Original Response: {}", body);
                        }
                    }
                }
                Some(body)
            }
            Err(e) => {
                error!("Failed to parse JSON response: {}", e);
                None
            }
        }
    }

    async fn direct_send(&self, start: Instant, body: &Value) -> Option<Value> {
        let req = self.build_request(body);
        let output = Self::send(req).await;
        info!("Call took: {} ms", start.elapsed().as_millis());
        output
    }

    async fn check_cache(&self, start: Instant, body: &Value) -> Option<Option<Value>> {
        if let Ok(key) = serde_json::to_vec(&body) {
            return Some(if let Some(value) = self.cache.get(&key).await {
                let sleep_duration = Duration::from_millis(1000);
                sleep(sleep_duration).await;
                info!(
                    "Cache hit: {} us sleep: {} us",
                    start.elapsed().as_micros(),
                    sleep_duration.as_micros()
                );

                Some(value)
            } else {
                let value = self.direct_send(start, body).await;
                if let Some(ref ref_value) = value {
                    self.cache.insert(key, ref_value.clone()).await;
                }
                value
            });
        }
        None
    }
}
#[async_trait]
impl ReDucter for LLmCaller {
    async fn call(&self, model: &str, prompt: &str, message: &str) -> Option<Value> {
        let start = Instant::now();
        let body = self.build_body(model, prompt, message);
        if tracing::enabled!(Level::DEBUG) {
            let pretty_body = pretty(&body);
            debug!(
                "Request body size: {} body: {}\n endpoint: {}",
                pretty_body.len(),
                pretty_body,
                self.endpoint
            );
        }
        if self.cache_flag {
            if let Some(value) = self.check_cache(start, &body).await {
                return value;
            }
        }
        self.direct_send(start, &body).await
    }
}

fn pretty(body: &Value) -> String {
    serde_json::to_string_pretty(&body).unwrap_or_default() //
}
