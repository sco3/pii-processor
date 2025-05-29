use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tracing::info;

/// structure for toggle ready/not ready for probe
#[derive(Clone)]
pub struct Toggle {
    /// name of the toggle
    name: String,
    /// ready flag
    is_ready: Arc<AtomicBool>,
}

impl Toggle {
    /// constructor
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            is_ready: Arc::new(AtomicBool::new(false)),
        }
    }
    /// ready setter
    pub fn set_ready(&self, ready: bool) {
        self.is_ready.store(ready, Ordering::Relaxed);
        info!("Component '{}' readiness set to {}", self.name, ready);
    }
    /// readiness getter
    #[must_use]
    pub fn is_ready(&self) -> bool {
        self.is_ready.load(Ordering::Relaxed)
    }
    /// inverse readiness getter
    #[must_use]
    pub fn not_ready(&self) -> bool {
        !self.is_ready.load(Ordering::Relaxed)
    }
    /// name getter
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }
}
