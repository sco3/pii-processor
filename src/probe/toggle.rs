use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone)]
pub struct Toggle {
    name: String,
    is_ready: Arc<AtomicBool>,
}

impl Toggle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_ready: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn set_ready(&self, ready: bool) {
        self.is_ready.store(ready, Ordering::Relaxed);
        println!("Component '{}' readiness set to {}", self.name, ready);
    }

    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
