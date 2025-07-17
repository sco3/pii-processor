use ductaper::llm_work::prompt::read_prompt;
use ductaper::util::logging::init_tracing;

#[test]
#[should_panic]
fn test_should_panic() {
    init_tracing();
    read_prompt("/tmp/asdf.asdf.txt", false);
}
