use ductaper::util::init::Init;
use ductaper::util::starter::Starter;

#[tokio::main]
async fn main() {
    Starter::new().await.start().await;
}
