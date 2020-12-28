#![forbid(unsafe_code)]

pub mod source;

mod lexer;
pub use lexer::Lexer;

mod static_environment;
pub use static_environment::StaticEnvironment;

pub(crate) mod parse_value;

mod parser_options;
pub use parser_options::ParserOptions;

mod parser_result;
pub use parser_result::ParserResult;

mod parser;
pub(crate) use parser::Loc;
pub use parser::{token_name, Parser};

mod builder;
pub(crate) use builder::Builder;

mod current_arg_stack;
pub(crate) use current_arg_stack::CurrentArgStack;

mod max_numparam_stack;
pub(crate) use max_numparam_stack::MaxNumparamStack;

mod variables_stack;
pub(crate) use variables_stack::VariablesStack;

mod error;
pub use error::{Diagnostic, DiagnosticMessage, ErrorLevel};

pub(crate) mod maybe_byte;

mod lex_state;
pub use lex_state::lex_states;
pub(crate) use lex_state::LexState;

mod token_buf;
pub(crate) use token_buf::TokenBuf;

mod reserved_words;
pub use reserved_words::reserved_word;

mod stack_state;
pub(crate) use stack_state::StackState;

pub(crate) mod str_term;

mod context;
pub(crate) use context::{Context, ContextItem};

pub mod nodes;
pub use nodes::Node;

pub mod traverse;

mod string_value;
pub use string_value::StringValue;

mod token;
pub use token::{Token, TokenValue};

pub mod token_rewriter;
