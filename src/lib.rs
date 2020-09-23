#![feature(label_break_value)]

extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;

pub mod lexer;
pub use lexer::Lexer;

pub mod meta;

mod messages;
pub use messages::Message;

mod static_environment;
pub use static_environment::StaticEnvironment;

mod parser;
pub use parser::{Parser, Loc, SymbolKind, Token};

#[cfg(test)]
mod tests {
    fn test() -> Vec<i32> {
        let mut v1 = vec![1, 2, 3];
        let mut v2 = vec![4, 5, 6];
        let v3 = [v1, v2].concat();
        v3
    }

    #[test]
    fn test_test() {
        assert_eq!(test(), vec![])
    }
}
