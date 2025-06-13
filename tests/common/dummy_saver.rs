use async_trait::async_trait;
use ductaper::data::session_log_models::SessionLog;
use ductaper::storage::saver::Saver;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

pub struct DummySaver {
    pub count: Arc<AtomicU32>,
}

impl DummySaver {
    pub fn new() -> Self {
        DummySaver {
            count: Arc::new(AtomicU32::new(0)),
        }
    }
}
#[async_trait]
impl Saver for DummySaver {
    async fn save(&self, _log: SessionLog, _file_name: &str) -> bool {
        self.count.fetch_add(1, Ordering::Relaxed);
        true
    }
}
