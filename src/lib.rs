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

mod parser;
pub use parser::{Parser, Loc, SymbolKind, Token};

pub mod node;
pub use node::Node;
mod builder;
pub use builder::Builder;
pub mod map_builder;

mod current_arg_stack;
pub use current_arg_stack::CurrentArgStack;

mod max_numparam_stack;
pub use max_numparam_stack::MaxNumparamStack;

mod variables_stack;
pub use variables_stack::VariablesStack;

mod error;
pub use error::{ParseError, ErrorLevel, ErrorMessage};

pub mod lex_char;

mod lex_state;
pub use lex_state::{LexState, lex_states};

mod token_buf;
pub use token_buf::TokenBuf;

mod reserved_words;
pub use reserved_words::reserved_word;

mod stack_state;
pub use stack_state::StackState;

pub mod str_term;

mod context;
pub use context::{Context, ContextItem};
