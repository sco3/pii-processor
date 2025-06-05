use crate::init::Init;
use ductaper::s3helper::S3Helper;
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
    //Starter::new(None).init().start();
    if let Ok(s3) = S3Helper::new(
        "test-bucket".to_string(),
        "eu-west-1".to_string(), //
        None,
        None,
        None,
        None,
    )
    .await
    {
        s3.list_buckets().await;
    }
}
