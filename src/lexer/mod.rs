#[allow(dead_code, non_snake_case)]
mod lexer;
pub use lexer::Lexer;

#[allow(dead_code)]
mod str_term;
pub use str_term::{StrTerm, StringLiteral, HeredocLiteral};

#[allow(dead_code)]
mod lex_state;
pub use lex_state::{LexState, lex_states};

#[allow(dead_code)]
mod locals_table;
pub use locals_table::LocalsTable;

#[allow(dead_code)]
mod lex_context;
pub use lex_context::LexContext;

#[allow(non_camel_case_types)]
mod token_type;
pub use token_type::TokenType;

#[allow(dead_code, non_upper_case_globals)]
mod strings;
