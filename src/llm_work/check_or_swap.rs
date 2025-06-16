/// Swaps key and value if key is wrapped in [] or appears to be a masked value (ends with ****).
/// Returns (unmasked_value, identifier) tuple.
pub fn check_or_swap<'a>(key: &'a str, value: &'a str) -> (&'a str, &'a str) {
    if key.starts_with("[") && key.ends_with("]") {
        return (value, key);
    }
    if key.len() > 4
        && key.ends_with("****")
        && !key.starts_with('*')
        && key.chars().nth(1) != Some('*')
        && key.chars().nth(2) != Some('*')
        && key.chars().nth(3) != Some('*')
    {
        return (value, key);
    }
    (key, value)
}

/// Tests for check_or_swap function
#[test]
pub fn test_swap() {
    assert_eq!(
        check_or_swap("a", "[b]"), //
        ("a", "[b]"),
    );
    assert_eq!(
        check_or_swap("[a]", "b"), //
        ("b", "[a]"),
    );
    assert_eq!(
        check_or_swap("asdf****", "b"), //
        ("b", "asdf****"),
    );
    assert_eq!(
        check_or_swap("b", "asdf****"), //
        ("b", "asdf****")
    );
    assert_eq!(
        check_or_swap("[PERSON]", "Joulie Yen"),
        ("Joulie Yen", "[PERSON]"),
    );
    assert_eq!(
        check_or_swap("Joulie Yen", "[PERSON]"),
        ("Joulie Yen", "[PERSON]"),
    );
    assert_eq!(
        check_or_swap("user@example.com", "user@e**********"),
        ("user@example.com", "user@e**********"),
    );
    assert_eq!(
        check_or_swap("user@e**********", "user@example.com"),
        ("user@example.com", "user@e**********"),
    );
    assert_eq!(
        check_or_swap("1234 5678 1234 5678", "1234 **** **** ****"),
        ("1234 5678 1234 5678", "1234 **** **** ****"),
    );
    assert_eq!(
        check_or_swap("1234 **** **** ****", "1234 5678 1234 5678"),
        ("1234 5678 1234 5678", "1234 **** **** ****"),
    );
}