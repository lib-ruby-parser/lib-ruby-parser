crate::use_native_or_external!(List);

use super::DecodedInput;
use crate::source::SourceLine;

#[test]
fn test_new() {
    let decoded = DecodedInput::named("foo");
    assert_eq!(decoded.name(), "foo");
}

fn decoded_input() -> DecodedInput {
    let mut decoded = DecodedInput::named("foo");
    decoded.set_bytes(List::from(vec![1, 2, 3]));
    decoded.set_lines(List::from(vec![SourceLine::new(1, 2, true)]));
    decoded
}

#[test]
fn test_settter() {
    let decoded = decoded_input();

    assert_eq!(decoded.bytes(), &vec![1, 2, 3]);
    assert_eq!(decoded.lines(), &vec![SourceLine::new(1, 2, true)]);
}

#[test]
fn test_debug() {
    let decoded = decoded_input();

    assert_eq!(
        format!("{:?}", decoded),
        "DecodedInput { name: \"foo\", lines: [SourceLine { start: 1, end: 2, ends_with_eof: true }], bytes: [1, 2, 3] }"
    );
}

#[test]
fn test_take_bytes() {
    let mut decoded = decoded_input();

    assert_eq!(decoded.take_bytes(), vec![1, 2, 3]);
    assert_eq!(decoded.take_bytes(), vec![]);
}

#[test]
fn test_into_bytes() {
    let decoded = decoded_input();

    assert_eq!(decoded.into_bytes(), vec![1, 2, 3]);
}
