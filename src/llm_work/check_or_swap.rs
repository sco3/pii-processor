pub fn check_or_swap<'a>(key: &'a str, value: &'a str) -> (&'a str, &'a str) {
    if key.starts_with("[") && key.ends_with("]") {
        return (value, key);
    }
    if key.len() > 4
        && key.ends_with("****")
        && key.chars().nth(0) != Some('*')
        && key.chars().nth(1) != Some('*')
        && key.chars().nth(2) != Some('*')
        && key.chars().nth(3) != Some('*')
    {
        return (value, key);
    }
    (key, value)
}

#[test]
pub fn test_swap() {
    assert_eq!(("a", "[b]"), check_or_swap("a", "[b]"));
    assert_eq!(("b", "[a]"), check_or_swap("[a]", "b"));
    assert_eq!(("b", "asdf****"), check_or_swap("asdf****", "b"));
    assert_eq!(("b", "asdf****"), check_or_swap("b", "asdf****"));
}
