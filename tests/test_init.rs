use ductaper::init::Init;
use ductaper::starter::Starter;

#[test]
fn test_init() {
    let starter = Starter::new(None);
    starter.init();
    starter.start();
}
