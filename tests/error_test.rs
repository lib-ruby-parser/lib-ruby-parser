use ruby_parser::{source::buffer::*, source::Range, ErrorLevel, ErrorMessage, ParseError};
use std::rc::Rc;

#[test]
fn it_renders() {
    let source = "line 1\nvery long line 2";
    let buffer = Buffer::new("(test_render)", source.as_bytes().to_vec(), None).unwrap();

    let error = ParseError::new(
        ErrorLevel::Warning,
        ErrorMessage::CvarName {
            name: "@@foo".to_owned(),
        },
        Range::new(8, 12, Rc::clone(&buffer.input)),
    );

    assert_eq!(
        error.render(&buffer).unwrap(),
        vec![
            "(test_render):2:1: warning: `@@foo' is not allowed as a class variable name",
            "(test_render):2: very long line 2",
            "(test_render):2: ^~~"
        ]
        .join("\n")
    );
}
