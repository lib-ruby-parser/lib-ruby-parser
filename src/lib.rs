#![feature(label_break_value)]

extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;

pub mod lexer;
pub use lexer::{Lexer, Context};

pub mod meta;

mod messages;
pub use messages::Message;

mod static_environment;
pub use static_environment::StaticEnvironment;

mod parser;
pub use parser::{Parser, Loc, SymbolKind, Token};

pub mod node;
pub use node::Node;
mod builder;
pub use builder::Builder;

mod current_arg_stack;
pub use current_arg_stack::CurrentArgStack;
