use ductaper::init::Init;
use ductaper::starter::Starter;

#[tokio::main]
async fn main() {
    Starter::new().await.init().start().await;
}
