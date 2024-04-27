#![no_std]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(deprecated_in_future)]
#![warn(unused_lifetimes)]
#![allow(clippy::boxed_local)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

/*!
A Ruby parser written in Rust.

Uses bison under the hood.
*/

// Re-exporting lib_ruby_parser_ast
/// Module with all known node types
pub use lib_ruby_parser_ast::nodes;
pub use lib_ruby_parser_ast::ByteArray;
pub use lib_ruby_parser_ast::DiagnosticMessage;
pub use lib_ruby_parser_ast::Loc;
pub use lib_ruby_parser_ast::Node;

mod loc_ext;
pub use loc_ext::LocExt;

/// Module with everything related to output of the Parser, but not related to AST,
/// like `Comment`, `Input`, `Decoder`
pub mod source;

#[allow(clippy::collapsible_if)]
#[allow(clippy::collapsible_else_if)]
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
pub use parser::{Parser, YYStackItem};

#[allow(dead_code, unused_variables)]
mod builder;
pub(crate) use builder::Builder;

mod current_arg_stack;
pub(crate) use current_arg_stack::CurrentArgStack;

mod max_numparam_stack;
pub(crate) use max_numparam_stack::MaxNumparamStack;

mod variables_stack;
pub(crate) use variables_stack::VariablesStack;

mod error;
pub use error::{Diagnostic, ErrorLevel};

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
pub(crate) use context::SharedContext;

/// Module to perform recursive traversing
pub use lib_ruby_parser_ast::traverse;
mod token;
pub use token::Token;

#[cfg(test)]
mod tests;
