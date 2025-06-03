use crate::init::Init;
use crate::starter::Starter;

pub mod env_vars;
pub mod init;
pub mod logging;
pub mod secret_string;
pub mod starter;

fn main() {
    let starter = Starter::new(None);
    starter.init();
    starter.start();
}
