use ruby_parser::source::buffer::{Buffer, BufferError};

#[test]
fn it_detects_explicit_koi_r() {
    let buffer = Buffer::new_from_file("test.rb", "tests/source/fixtures/valid/koi8_r.rb").unwrap();

    assert_eq!(buffer.name(), "test.rb");
    assert_eq!(buffer.lines, vec![
        String::from("# encoding: koi8-r"),
        String::from(""),
        String::from("s = \'привет\'"),
    ]);
    assert_eq!(buffer.encoding(), "koi8-r")
}

#[test]
fn it_detects_explicit_utf_8() {
    let buffer = Buffer::new_from_file("test.rb", "tests/source/fixtures/valid/utf_8.rb").unwrap();

    assert_eq!(buffer.name(), "test.rb");
    assert_eq!(buffer.lines, vec![
        String::from("# coding: utf-8"),
        String::from(""),
        String::from("\"🥰\""),
    ]);
    assert_eq!(buffer.encoding(), "utf-8")
}

#[test]
fn it_detects_fallback_utf_8() {
    let buffer = Buffer::new_from_file("test.rb", "tests/source/fixtures/valid/unknown.rb").unwrap();

    assert_eq!(buffer.name(), "test.rb");
    assert_eq!(buffer.lines, vec![
        String::from("42"),
    ]);
    assert_eq!(buffer.encoding(), "utf-8")
}

#[test]
fn it_handles_incorrect_encoding_comment() {
    let err = Buffer::new_from_file("test.rb", "tests/source/fixtures/invalid/incorrect_encoding_comment.rb").unwrap_err();

    assert_eq!(err, BufferError::EncodingError("invalid sequence".into()));
}

#[test]
fn it_handles_no_encoding_comment_not_utf8() {
    let err = Buffer::new_from_file("test.rb", "tests/source/fixtures/invalid/no_encoding_comment_not_utf8.rb").unwrap_err();

    assert_eq!(err, BufferError::UnrecognizedEncoding);
}

#[test]
fn it_handles_missing_file() {
    let err = Buffer::new_from_file("test.rb", "missing.rb").unwrap_err();

    assert_eq!(err, BufferError::InputFileDoesNotExit)
}

fn make_test_buffer() -> Buffer {
    Buffer::new_from_source("(eval)", "100 + 200").unwrap()
}


#[test]
fn it_returns_subsource_using_slice_method() {
    let test_buffer = make_test_buffer();

    assert_eq!(test_buffer.slice(1..5), Some(&['0', '0', ' ', '+'] as &[char]));
    assert_eq!(test_buffer.slice(100..200), None);
}
