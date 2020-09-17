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

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub parser); // synthesized by LALRPOP

#[cfg(test)]
mod test {
    use super::parser::ProgramParser;
    use super::Lexer;

    #[test]
    fn test_parser() {
        let parser = ProgramParser::new();
        let lexer = Lexer::new("1_000_000 + 2");
        let a = parser.parse(lexer.into_iter()).unwrap();
        println!("{:#?}", a);
        assert_eq!(3, 4);
    }
}
