use ductaper::logging::init_log;

#[test]
fn test_logging() {
    init_log(&"info".to_string());
}
