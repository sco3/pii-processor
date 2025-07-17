use ductaper::util::exit_codes::ExitCode;

#[test]
pub fn test_exit_codes() {
    println!("{}", ExitCode::Success.code());
}
