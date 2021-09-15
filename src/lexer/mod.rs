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
