use ductaper::starter::Starter;
use ductaper::util::init::Init;
use std::env::args;

#[tokio::main]
async fn main() {
    let a = args().nth(1).unwrap_or_default();
    // start tokio-console in other terminal to look at the metrics
    // run the application with [-c]
    if a == "-c" {
        console_subscriber::init();
    }

    Starter::new().await.start().await;
}
