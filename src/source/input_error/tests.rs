use super::InputError;

#[test]
fn test_unsupported_encoding() {
    let err = InputError::new_unsupported_encoding(String::from("foo"));

    assert!(err.is_unsupported_encoding());
    assert!(!err.is_decoding_error());

    assert_eq!(err.get_unsupported_encoding_message(), "foo");
    assert_eq!(format!("{:?}", err), "UnsupportedEncoding(\"foo\")");
}

#[test]
fn test_decoding_error() {
    let err = InputError::new_decoding_error(String::from("bar"));

    assert!(err.is_decoding_error());
    assert!(!err.is_unsupported_encoding());

    assert_eq!(err.get_decoding_error_message(), "bar");
    assert_eq!(format!("{:?}", err), "DecodingError(\"bar\")");
}
