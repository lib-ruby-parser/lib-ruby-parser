use lib_ruby_parser::ParserResult;

#[derive(Clone)]
pub(crate) struct Printer {
    f: fn(&ParserResult),
}

impl std::fmt::Debug for Printer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Printer").finish()
    }
}

mod formatters {
    use super::ParserResult;

    pub(crate) fn print_only_diagnostics(result: &ParserResult) {
        for d in result.diagnostics.iter() {
            println!(
                "{}",
                d.render(&result.input)
                    .expect("Failed to render a diagnostic")
            )
        }
    }

    pub(crate) fn print_nothing(_: &ParserResult) {}

    pub(crate) fn print_compact_ast_with_locations(result: &ParserResult) {
        let src = result.input.as_shared_bytes();
        let src = std::str::from_utf8(src).unwrap_or("invalid-source");
        println!("{}", src);
        print_only_diagnostics(result);
        if let Some(ast) = result.ast.as_ref() {
            ast.print_with_locs()
        }
    }
    pub(crate) fn print_compact_ast(result: &ParserResult) {
        print_only_diagnostics(result);
        if let Some(ast) = result.ast.as_ref() {
            println!("{}", ast.inspect(0));
        }
    }
    pub(crate) fn print_full_ast(result: &ParserResult) {
        println!("{:#?}", result)
    }
}

impl Printer {
    pub const ABOUT: &'static str =
        "N = Nothing, F = Full AST, L = Compact AST with locations, D = Only Diagnostics, default = Compact AST";

    pub(crate) fn new(f: fn(&ParserResult)) -> Self {
        Self { f }
    }

    pub(crate) fn print(&self, result: &ParserResult) {
        (self.f)(result)
    }
}

impl Default for Printer {
    fn default() -> Self {
        Self::new(formatters::print_compact_ast)
    }
}

impl std::str::FromStr for Printer {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let f = match s {
            "N" => formatters::print_nothing,
            "F" => formatters::print_full_ast,
            "L" => formatters::print_compact_ast_with_locations,
            "D" => formatters::print_only_diagnostics,
            _ => return Err(Self::ABOUT),
        };
        Ok(Self::new(f))
    }
}
