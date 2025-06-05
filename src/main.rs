use crate::init::Init;
use crate::starter::Starter;

pub mod ai_tags;
pub mod env_vars;
pub mod init;
pub mod llm_caller;
pub mod logging;
pub mod redact_consumer;
pub mod secret_string;
pub mod starter;
#[tokio::main]
async fn main() {
    Starter::new(None).await.init().start();
}
