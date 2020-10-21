#![feature(label_break_value)]

extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;

pub mod lexer;
pub use lexer::Lexer;

pub mod meta;

mod static_environment;
pub use static_environment::StaticEnvironment;

pub mod parse_value;

mod parser;
pub use parser::{Loc, Parser, SymbolKind, Token, TokenValue};

mod builder;
pub use builder::Builder;
pub mod map_builder;

mod current_arg_stack;
pub(crate) use current_arg_stack::CurrentArgStack;

mod max_numparam_stack;
pub(crate) use max_numparam_stack::MaxNumparamStack;

mod variables_stack;
pub(crate) use variables_stack::VariablesStack;

mod error;
pub use error::{ErrorLevel, ErrorMessage, ParseError};

pub mod maybe_byte;

mod lex_state;
pub use lex_state::lex_states;
pub(crate) use lex_state::LexState;

mod token_buf;
pub(crate) use token_buf::TokenBuf;

mod reserved_words;
pub(crate) use reserved_words::reserved_word;

mod stack_state;
pub(crate) use stack_state::StackState;

pub mod str_term;

mod context;
pub(crate) use context::{Context, ContextItem};

pub mod nodes;
pub use nodes::Node;

pub mod traverse;
