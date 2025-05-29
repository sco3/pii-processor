use async_trait::async_trait;
use ductaper::llm_work::reducter::ReDucter;
use ductaper::worker_pool::serve::Stat;
use serde_json::{Value, json};
use tracing::debug;

pub struct DummyCaller {
    response: Option<Value>,
}

impl DummyCaller {
    #[allow(dead_code)]
    pub fn new(response: Option<&str>) -> Self {
        if let Some(r) = response {
            let v = serde_json::from_slice(r.as_ref()).unwrap();
            debug!("Dummy response: {:?}", v);
            return DummyCaller { response: v };
        }
        DummyCaller { response: None }
    }
}
#[async_trait]
impl ReDucter for DummyCaller {
    async fn call(
        &self,
        _model: &str,
        _prompt: &str,
        _message: &str,
        _stat: &mut Stat,
    ) -> Option<Value> {
        debug!("call");
        if let Some(v) = self.response.clone() {
            return Some(v);
        }
        Some(json!({}))
    }
}
