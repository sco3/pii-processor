use crate::llm_work::llm_caller::LLmCaller;
use crate::llm_work::preview::preview_str;
use crate::worker_pool::serve::Stat;
use reqwest::{RequestBuilder, Response};
use serde_json::Value;
use std::time::Instant;
use tracing::{Level, debug, error};

/// lite llm duration header
const DUR_HDR: &str = "x-litellm-response-duration-ms";
/// lite llm overhead hdeader
const OHEAD_HDR: &str = "x-litellm-overhead-duration-ms";

impl LLmCaller {
    /// Sends request and processes response
    pub async fn send(req: RequestBuilder, message: &str, stat: &mut Stat) -> Option<Value> {
        let start_send = Instant::now();
        let resp = match req.send().await {
            Ok(resp) => {
                stat.send_micros = start_send.elapsed().as_micros();
                resp
            }
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

        let litellm_took = Self::get_header(
            &resp, //
            DUR_HDR,
        );
        stat.extra_info
            .insert(DUR_HDR.to_string(), litellm_took.to_string());

        let litellm_ov = Self::get_header(
            &resp, //
            OHEAD_HDR,
        );
        stat.extra_info
            .insert(OHEAD_HDR.to_string(), litellm_ov.to_string());

        debug!("Reported {DUR_HDR}: {litellm_took} {OHEAD_HDR}: {litellm_ov}");

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

    fn get_header<'a>(resp: &'a Response, name: &str) -> &'a str {
        resp.headers()
            .get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("0")
    }

    /// Sends request directly (no cache)
    pub(crate) async fn direct_send(
        &self, //
        start_processing: Instant,
        body: &Value,
        message: &str,
        stat: &mut Stat,
    ) -> Option<Value> {
        // in cortex llm is called three times, let's do the same.
        for attempt in 0..3 {
            let req = self.build_request(body);
            let start_send = Instant::now();
            if let Some(v) = Self::send(req, message, stat).await {
                stat.build_and_call = start_send.elapsed().as_micros();
                debug!(
                    "Build&call: {} us send: {} us attempt: {}",
                    start_send.elapsed().as_micros(),
                    start_processing.elapsed().as_micros(),
                    attempt
                );
                return Some(v);
            }
        }
        None
    }
}
