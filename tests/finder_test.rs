use lib_ruby_parser::traverse::finder::Finder;
use lib_ruby_parser::{debug_level, Parser, ParserOptions, ParserResult};

fn find(src: &str, pattern: &str) -> Option<String> {
    let options = ParserOptions {
        buffer_name: "(find_test)".to_owned(),
        debug: debug_level::NONE,
        ..Default::default()
    };
    let parser = Parser::new(src.as_bytes(), options);

    let ParserResult { ast, input, .. } = parser.do_parse();
    let ast = ast.expect("expected AST to be Some");
    let node = Finder::run(&pattern, &ast).unwrap()?;
    node.expression().source(&input)
}

#[test]
fn it_finds() {
    let src = "[1,2,3].each { |a| puts a + 1; 42 }";
    let pattern = "root -> body -> stmts -> 0 -> args -> 0";

    assert_eq!(Some("a + 1".to_owned()), find(src, pattern))
}

#[test]
fn it_returns_none_if_no_node() {
    let src = "[1,2,3]";
    let pattern = "body";

    assert_eq!(None, find(src, pattern))
}
