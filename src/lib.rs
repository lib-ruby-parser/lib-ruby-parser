#![feature(label_break_value)]

extern crate encoding;
extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod source;

pub mod lexer;
pub use lexer::State;
pub use lexer::Lexer;

pub mod meta;

mod messages;
pub use messages::Message;

mod static_environment;
pub use static_environment::StaticEnvironment;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser);

#[cfg(test)]
mod test {
    use super::parser::ProgramParser;
    use super::{State, Lexer};
    use std::rc::Rc;
    use std::cell::RefCell;

    #[test]
    fn test_parser() {
        let parser = ProgramParser::new();
        let state = Rc::new(RefCell::new(State::new("1_000_000 + 2")));
        let lexer = Lexer::new(Rc::clone(&state));
        let a = parser.parse(&state, lexer.into_iter()).unwrap();
        println!("{:#?}", a);
        assert_eq!(3, 4);
    }
}
