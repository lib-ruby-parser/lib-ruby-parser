use ruby_parser::{ErrorLevel, ErrorMessage, ParseError, Buffer, BufferEncoding, source::Range};

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2";
    let buffer = Buffer::new(
        "(test_render)",
        source.as_bytes().to_vec(),
        BufferEncoding::Unknown
    );

    let error = ParseError::new(
        ErrorLevel::Warning,
        ErrorMessage::CvarName { name: "@@foo".to_owned() },
        Range::new(8, 12)
    );

    assert_eq!(
        error.render(&buffer).unwrap(),
        vec![
            "(test_render):2:1: warning: `@@foo' is not allowed as a class variable name",
            "(test_render):2: very long line 2",
            "(test_render):2: ^~~"
        ].join("\n")
    );
}
