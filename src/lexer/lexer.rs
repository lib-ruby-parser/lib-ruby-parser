use std::rc::Rc;
use std::cell::RefCell;
use crate::lexer::{State, Token};

pub struct Lexer {
    state: Rc<RefCell<State>>
}

impl Lexer {
    pub fn new(state: Rc<RefCell<State>>) -> Self {
        Self { state }
    }
}

#[derive(Debug)]
pub struct LexError {}

impl Iterator for Lexer {
    type Item = Result<(usize, Token, usize), LexError>;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.state.borrow_mut().yylex();
        match token {
            Token::END_OF_INPUT(..) => None,
            _ => {
                let begin = *token.begin();
                let end = *token.end();
                Some(Ok((begin, token, end)))
            }
        }
    }
}
