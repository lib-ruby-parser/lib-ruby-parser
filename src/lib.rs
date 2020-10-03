#![feature(label_break_value)]

extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;
pub use source::{Buffer, BufferEncoding};

pub mod lexer;
pub use lexer::{Lexer, Context};

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
