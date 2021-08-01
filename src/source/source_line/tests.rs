use super::SourceLine;

fn source_line() -> SourceLine {
    SourceLine::new(1, 2, true)
}

#[test]
fn test_new() {
    let line = source_line();
    drop(line)
}

#[test]
fn test_start() {
    let line = source_line();
    assert_eq!(line.start(), 1)
}

#[test]
fn test_end() {
    let line = source_line();
    assert_eq!(line.end(), 2)
}

#[test]
fn test_ends_with_eof() {
    let line = source_line();
    assert_eq!(line.ends_with_eof(), true)
}

#[test]
fn test_set_start() {
    let mut line = source_line();
    line.set_start(10);
    assert_eq!(line.start(), 10)
}

#[test]
fn test_set_end() {
    let mut line = source_line();
    line.set_end(20);
    assert_eq!(line.end(), 20)
}

#[test]
fn test_set_ends_with_eof() {
    let mut line = source_line();
    line.set_ends_with_eof(false);
    assert_eq!(line.ends_with_eof(), false)
}

#[test]
fn test_debug() {
    let line = source_line();
    assert_eq!(
        format!("{:?}", line),
        "SourceLine { start: 1, end: 2, ends_with_eof: true }"
    )
}
