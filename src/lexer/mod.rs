#[cfg(feature = "debug-lexer")]
macro_rules! println_if_debug_lexer {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr();
        stderr.write_fmt(core::format_args!($($arg)*)).unwrap();
        stderr.write(b"\n").unwrap();
    }};
}
#[cfg(not(feature = "debug-lexer"))]
macro_rules! println_if_debug_lexer {
    () => {};
    ($($arg:tt)*) => {{}};
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
