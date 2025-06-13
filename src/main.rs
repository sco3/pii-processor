use ductaper::util::init::Init;
use ductaper::starter::Starter;

#[tokio::main]
async fn main() {
    Starter::new().await.start().await;
}
