use crate::config::env_vars::Cfg;
use crate::data::ai_tags::Ai;
use mime::APPLICATION_JSON;
use moka::sync::Cache;
use reqwest::RequestBuilder;
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;
use serde_json::json;
use std::sync::Arc;
use tracing::{Level, debug};

/// Client for making LLM API calls with caching support
pub struct LLmCaller {
    /// API endpoint URL
    pub endpoint: String,
    /// Default model name
    pub model: String,
    /// Optional bearer token for authentication
    pub bearer: Option<String>,
    /// HTTP client instance
    pub client: reqwest::Client,
    /// Flag to enable/disable caching
    pub cache_flag: bool,
    /// Response cache storage
    pub(crate) cache: Cache<Arc<str>, Value>,
    /// emulate work with cache
    pub(crate) cache_sleep_millis: u64,
    /// configuration
    pub cfg: Cfg,
}

impl LLmCaller {
    /// Formats bearer token with prefix
    pub(crate) fn add_bearer(token: Option<&String>) -> Option<String> {
        token.as_ref().map(|t| format!("{} {}", Ai::BEARER, t))
    }

    /// Constructs request body JSON
    pub(crate) fn build_body(model: &str, prompt: &str, message: &str) -> Value {
        json!({
            Ai::MODEL: model,
            Ai::MESSAGES: [
                { Ai::ROLE: Ai::SYSTEM, Ai::CONTENT: prompt },
                { Ai::ROLE: Ai::USER, Ai::CONTENT: message }
            ],
            Ai::TEMPERATURE: 0
        })
    }

    /// Builds HTTP request with headers and body
    pub fn build_request(&self, body: &Value) -> RequestBuilder {
        let mut req = self
            .client
            .post(&self.endpoint)
            .header(CONTENT_TYPE, APPLICATION_JSON.as_ref())
            .body(body.to_string());

        if let Some(ref bearer) = self.bearer {
            req = req.header(AUTHORIZATION, bearer);
        }

        if let (Some(h), Some(v)) = (
            &self.cfg.portkey_aws_region_header, //
            &self.cfg.aws_region,
        ) {
            req = req.header(h, v);
        }

        if let (Some(h), Some(v)) = (
            &self.cfg.portkey_provider_header, //
            &self.cfg.portkey_provider_value,
        ) {
            req = req.header(h, v);
        }

        if let (Some(h), Some(v)) = (
            &self.cfg.portkey_aws_secret_access_key_header, //
            &self.cfg.aws_secret_access_key,
        ) {
            req = req.header(h, v.get_string());
        }

        if let (Some(h), Some(v)) = (
            &self.cfg.portkey_aws_access_token_header, //
            &self.cfg.aws_access_token,
        ) {
            req = req.header(h, v.get_string());
        }
        if let (Some(h), Some(v)) = (
            &self.cfg.portkey_aws_access_key_id_header, //
            &self.cfg.aws_access_key_id,
        ) {
            req = req.header(h, v.get_string());
        }

        req
    }

    /// write debug message with body preview if debug level is enabled
    pub(crate) fn debug_body(&self, body: &Value) {
        if tracing::enabled!(Level::DEBUG) {
            let pretty_body = pretty(body);
            debug!(
                "Request body size: {} body: {}\n endpoint: {}",
                pretty_body.len(),
                pretty_body,
                self.endpoint
            );
        }
    }
}

/// Formats JSON value for pretty printing
fn pretty(body: &Value) -> String {
    serde_json::to_string_pretty(&body).unwrap_or_default() //
}
