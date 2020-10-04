mod lexer;
pub use lexer::Lexer;

mod parse_ident;
mod parse_magic_comment;
mod parse_numeric;
mod parse_string;
mod tokadd;
pub use tokadd::TokAdd;
