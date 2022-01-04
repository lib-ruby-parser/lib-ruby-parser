macro_rules! println_if_debug_lexer {
    ($fmt_string:expr, $( $arg:expr ),*) => {
        if cfg!(feature = "debug-lexer") {
            println!($fmt_string, $( $arg ),*);
        }
    };
}
pub(crate) use println_if_debug_lexer;

mod main;
mod parse_atmark;
mod parse_gvar;
mod parse_heredoc;
mod parse_ident;
mod parse_magic_comment;
mod parse_numeric;
mod parse_percent;
mod parse_qmark;
mod parse_string;
mod tokadd;
mod yylval;

pub use main::Lexer;
pub(crate) use tokadd::TokAdd;

#[cfg(test)]
mod tests;
