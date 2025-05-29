use ductaper::starter::Starter;
use ductaper::util::init::Init;

#[tokio::main]
async fn main() {
    Starter::new().await.start().await;
}
