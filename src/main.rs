use crate::init::Init;
use crate::starter::Starter;

pub mod ai_tags;
pub mod connector;
pub mod env_vars;
pub mod expanduser;
pub mod init;
pub mod llm_caller;
pub mod llm_caller_trait;
pub mod llm_handler;
pub mod llm_work;
pub mod log_handler;
pub mod logging;
pub mod publisher;
pub mod redact_consumer;
pub mod secret_string;
pub mod session_log_models;
pub mod starter;

#[tokio::main]
async fn main() {
    Starter::new(None).await.init().start();
}
