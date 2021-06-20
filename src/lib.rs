#![cfg_attr(
    feature = "compile-with-external-structures",
    feature(allocator_api, slice_ptr_get)
)]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(deprecated_in_future)]
#![warn(unused_lifetimes)]

/*!
A Ruby parser written in Rust.

Uses bison under the hood.
*/

mod loc;
pub use loc::Loc;

/// Module with everything related to output of the Parser, but not related to AST,
/// like `Comment`, `Input`, `CustomDecoder`
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
pub use parser::Parser;

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
pub use lex_state::LexState;

mod token_buf;
pub(crate) use token_buf::TokenBuf;

mod reserved_words;
pub use reserved_words::{reserved_word, ReservedWord};

mod stack_state;
pub(crate) use stack_state::StackState;

pub(crate) mod str_term;

mod context;
pub(crate) use context::{Context, ContextItem};

/// Module with all known node types
pub mod nodes;
pub use nodes::Node;

/// Module to perform recursive traversing
pub mod traverse;

mod string_value;
pub use string_value::StringValue;

mod token;
pub use token::Token;

/// Module to perform token rewriting
pub mod token_rewriter;

mod bytes;
pub use bytes::Bytes;

/// Debug level of the parser
pub mod debug_level;

/// Module with generic containers
pub mod containers;
