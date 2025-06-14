use ductaper::starter::Starter;
use ductaper::util::init::Init;

use jemallocator::Jemalloc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[tokio::main]
async fn main() {
    Starter::new().await.start().await;
}
