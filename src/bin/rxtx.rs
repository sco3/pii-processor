use async_channel::bounded;
use tokio::task;

#[tokio::main]
async fn main() {
    let (tx, rx) = bounded::<String>(4);

    for i in 0..4 {
        let rx = rx.clone();
        task::spawn(async move {
            while let Ok(msg) = rx.recv().await {
                println!("Worker {i} got: {msg}");
            }
        });
    }

    for i in 0..10 {
        tx.send(format!("Task {i}")).await.unwrap();
    }
}
