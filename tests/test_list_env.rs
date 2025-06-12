use ductaper::list_env::list_env;
use std::env;

#[test]
pub fn test_list_env() {
    unsafe {
        env::set_var("AWS_1", "1");
    }
    list_env();
}
