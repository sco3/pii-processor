use ductaper::config::secret_string::SecretString;
#[test]
fn test_secret_string_values() {
    let asdf = SecretString::new("asdf-1234");
    assert_eq!(asdf.value, "asdf-1234");
    assert_eq!("asdf****", format!("{:?}", asdf));
    let short: SecretString = SecretString::new("12");
    assert_eq!(short.value, "12");
    assert_eq!("12****", format!("{:?}", short));
}
