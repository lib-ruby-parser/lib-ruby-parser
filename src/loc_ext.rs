use crate::source::DecodedInput;
pub use lib_ruby_parser_ast::Loc;

/// A set of parser-specific extensions for `Loc`
pub trait LocExt {
    /// Returns line and column of the `begin` of the `Loc` on a given `Input`
    fn begin_line_col(&self, input: &DecodedInput) -> Option<(usize, usize)>;
    /// Expands `Loc` to the whole line and returns line number and new `Loc`
    fn expand_to_line(&self, input: &DecodedInput) -> Option<(usize, Loc)>;
    /// Returns source code of the current `Loc` on a given `Input`
    fn source<'b>(&self, input: &DecodedInput<'b>) -> Option<&'b str>;
}

impl LocExt for Loc {
    fn begin_line_col(&self, input: &DecodedInput) -> Option<(usize, usize)> {
        input.line_col_for_pos(self.begin())
    }

    fn expand_to_line(&self, input: &DecodedInput) -> Option<(usize, Loc)> {
        let (begin_line, _) = self.begin_line_col(input)?;
        let line_no = begin_line;
        let line = input.line_at(line_no as u32);
        Some((line_no, Self::new(line.start, line.line_end())))
    }

    /// Returns source code of the current `Loc` on a given `Input`
    fn source<'b>(&self, input: &DecodedInput<'b>) -> Option<&'b str> {
        let bytes = input.substr_at(self.begin() as u32, self.end() as u32)?;
        core::str::from_utf8(bytes).ok()
    }
}
