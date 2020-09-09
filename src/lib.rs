extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;
pub mod lexer;
pub mod meta;
mod messages;
pub use messages::Message;
mod static_environment;
pub use static_environment::StaticEnvironment;
