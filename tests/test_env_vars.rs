use std::env;

use ductaper::env_vars::Cfg;
#[test]
fn test_values() {
    unsafe {
        env::set_var("NATS_URL", "NU");
        env::set_var("LLM_TOKEN", "LLM_TOKEN");
        env::set_var("LOG_LEVEL", "DEBUG");
    }
    let cfg = Cfg::from_env();
    assert_eq!(cfg.nats_url, "NU");
    assert_eq!(
        cfg.llm_token.get_string(),
        "LLM_TOKEN",
        "LLM token should match"
    );
    assert_eq!(cfg.log_level, "DEBUG");
    let redacted = format!("{:?}", cfg.llm_token);
    assert_eq!("LLM_****", redacted);
}

#[test]

fn test_from_env_missing_vars() {
    // Ensure no environment variables are set
    unsafe {
        env::remove_var("NATS_URL");
        env::remove_var("LOG_LEVEL");
        env::remove_var("LOG_FMT");
        env::remove_var("LLM_TOKEN");
    }
    // Call the method, expecting a panic
    Cfg::from_env();
}
