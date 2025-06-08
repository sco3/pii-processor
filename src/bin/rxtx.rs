use tokio::sync::mpsc::channel;
use tracing::debug;



fn main() {
    let (tx, rx) = channel::<u32>  (8);
    debug!("channel {:?} {:?}",tx,rx);
    
    
}