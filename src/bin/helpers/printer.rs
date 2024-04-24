use std::ffi::OsString;

use lib_ruby_parser::ParserResult;

#[derive(Clone, Debug)]
pub(crate) enum Printer {
    Nothing,
    FullAst,
    CompactAstWithLocations,
    Diagnostics,
    CompactAst,
}

impl Default for Printer {
    fn default() -> Self {
        Self::CompactAst
    }
}

impl From<OsString> for Printer {
    fn from(value: OsString) -> Self {
        match value.to_str().unwrap_or_default() {
            "N" => Self::Nothing,
            "F" => Self::FullAst,
            "L" => Self::CompactAstWithLocations,
            "D" => Self::Diagnostics,
            _ => panic!("Invalid printer argument, expected N, F, L, D or nothing"),
        }
    }
}

impl Printer {
    pub(crate) fn print(&self, result: &ParserResult) {
        match self {
            Self::Nothing => {}
            Self::FullAst => print_full_ast(result),
            Self::CompactAstWithLocations => print_compact_ast_with_locations(result),
            Self::Diagnostics => print_only_diagnostics(result),
            Self::CompactAst => print_compact_ast(result),
        }
    }
}

fn print_only_diagnostics(result: &ParserResult) {
    for d in result.diagnostics.iter() {
        let mut buf = String::new();
        d.render(&mut buf, &result.input).unwrap();
        println!("{}", buf);
    }
}

fn print_compact_ast_with_locations(result: &ParserResult) {
    let src = result.input.bytes;
    let src = std::str::from_utf8(src).unwrap_or("invalid-source");
    println!("{}", src);
    print_only_diagnostics(result);
    if let Some(ast) = result.ast.as_ref() {
        ast.print_with_locs()
    }
}

fn print_compact_ast(result: &ParserResult) {
    print_only_diagnostics(result);
    if let Some(ast) = result.ast.as_ref() {
        let mut buf = String::new();
        ast.inspect(0, &mut buf);
        println!("{}", buf);
    }
}

fn print_full_ast(result: &ParserResult) {
    println!("{:#?}", result)
}
