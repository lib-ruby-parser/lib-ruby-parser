use lib_ruby_parser::traverse::Find;
use lib_ruby_parser::{Parser, ParserOptions, ParserResult};

fn find(src: &str, pattern: Vec<&str>) -> Option<String> {
    let options = ParserOptions {
        buffer_name: "(find_test)".to_owned(),
        debug: false,
        ..Default::default()
    };
    let parser = Parser::new(src.as_bytes(), options);

    let pattern = pattern
        .into_iter()
        .map(|e| e.to_owned())
        .collect::<Vec<_>>();
    let ParserResult { ast, input, .. } = parser.do_parse();
    let node = Find::run(&pattern, &ast?).unwrap()?;
    node.expression().source(&input)
}

#[test]
fn it_finds() {
    let src = "[1,2,3].each { |a| puts a + 1; 42 }";
    let pattern = vec!["body", "stmt[0]", "arg[0]"];

    assert_eq!(Some("a + 1".to_owned()), find(src, pattern))
}

#[test]
fn it_returns_none_if_no_node() {
    let src = "[1,2,3]";
    let pattern = vec!["body"];

    assert_eq!(None, find(src, pattern))
}
