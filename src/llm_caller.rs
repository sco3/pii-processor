//use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use serde_json::json;

const _BEARER: &str = "Bearer";

pub struct LLmCaller {
    pub endpoint: String,
    pub model: String,
    pub token: Option<String>,
    pub client: reqwest::Client,
    pub bearer: Option<String>,
}

impl LLmCaller {
    pub fn _new(endpoint: String, model: String, token: Option<String>) -> Self {
        let client = reqwest::Client::new();
        let bearer = token
            .as_ref() //
            .map(|t| format!("{} {}", _BEARER, t));

        LLmCaller {
            endpoint,
            model,
            token,
            client,
            bearer,
        }
    }

    pub fn _call(&self, _prompt: &str, _message: &str) {
        let _body = json!({
            "model": json!(self.model),
            "messages": [
                { "role": "system", "content": ""},
                { "role": "user", "content": "" }
            ],
            "temperature":0
        });
    }
}
