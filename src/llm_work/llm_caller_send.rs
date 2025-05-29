use crate::llm_work::llm_caller::LLmCaller;
use crate::llm_work::preview::preview_str;
use reqwest::RequestBuilder;
use serde_json::Value;
use std::time::Instant;
use tracing::{Level, debug, error, info};

impl LLmCaller {
    /// Sends request and processes response
    pub async fn send(req: RequestBuilder, message: &str) -> Option<Value> {
        let resp = match req.send().await {
            Ok(resp) => resp,
            Err(e) => {
                error!("Request send failed: {:?}", e);
                return None;
            }
        };
        debug!("Call result status: {}", resp.status());

        if !resp.status().is_success() {
            let status = resp.status();
            let text = resp.text().await.ok();
            error!(
                "Request failed with status {}: {:?} {}",
                status,
                text,
                preview_str(message)
            );
            return None;
        }

        let litellm_took = resp
            .headers()
            .get("x-litellm-response-duration-ms")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("0");

        debug!("Litellm took: {} ms", litellm_took);

        match resp.json::<Value>().await {
            Ok(body) => {
                if tracing::enabled!(Level::DEBUG) {
                    if let Ok(pretty_body) = serde_json::to_string_pretty(&body) {
                        debug!("Response:\n{}", pretty_body);
                    } else {
                        debug!("Pretty print failed. Original Response: {}", body);
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

    /// Sends request directly (no cache)
    pub(crate) async fn direct_send(
        &self, //
        start: Instant,
        body: &Value,
        message: &str,
    ) -> Option<Value> {
        // in cortex llm is called three times, let's do the same.
        for attempt in 0..3 {
            let req = self.build_request(body);
            if let Some(v) = Self::send(req, message).await {
                info!(
                    "Call took: {} us attempt: {}",
                    start.elapsed().as_micros(),
                    attempt
                );
                return Some(v);
            }
        }
        None
    }
}
