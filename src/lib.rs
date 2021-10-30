/// Takes a `string`, and prefixes it with `prefix` unless it is already prefixed that way
pub fn prefix_string(prefix: &str, string: &str) -> String {
    let mut result: String = string.to_string();
    let prefix_length = prefix.len();
    let test_prefix: &str = &string[..prefix_length];
    if test_prefix != prefix {
        result = prefix.to_owned() + string;
    }
    result
}

#[test]
fn regular_usage() {
    let result = prefix_string("http://", "example.com");
    assert_eq!(result, "http://example.com")
}

#[test]
fn already_prefixed() {
    let result = prefix_string("ftp://", "ftp://example.com");
    assert_eq!(result, "ftp://example.com")
}
