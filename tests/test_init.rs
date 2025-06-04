use ductaper::init::Init;
use ductaper::starter::Starter;
use std::env;

#[test]
fn test_init() {
    unsafe {
        env::set_var("TENANT", "TENANT");
        env::set_var("APPLICATION", "APPLICATION");
        env::set_var("LLM_MODEL", "nova");
    }

    let starter = Starter::new(None);
    starter.init();
    starter.start();
}
