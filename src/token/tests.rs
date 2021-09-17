crate::use_native_or_external!(List);
use crate::{Bytes, LexState, Loc, Token};

fn lex_state(value: i32) -> LexState {
    let mut lex_state = LexState::default();
    lex_state.set(value);
    lex_state
}

fn new_token() -> Token {
    Token::new(
        1,
        Bytes::new(list![1, 2, 3]),
        Loc::new(1, 2),
        lex_state(1),
        lex_state(2),
    )
}

#[test]
fn test_new() {
    let token = new_token();
    drop(token);
}

#[test]
fn test_token_type() {
    let token = new_token();
    assert_eq!(token.token_type(), 1)
}

#[test]
fn test_token_value() {
    let token = new_token();
    assert_eq!(token.token_value(), &Bytes::new(list![1, 2, 3]));
}

#[test]
fn test_set_token_value() {
    let mut token = new_token();
    token.set_token_value(Bytes::new(list![4, 5, 6]));
    assert_eq!(token.token_value(), &Bytes::new(list![4, 5, 6]));
}

#[test]
fn test_into_token_value() {
    let token = new_token();
    assert_eq!(token.into_token_value(), Bytes::new(list![1, 2, 3]))
}

#[test]
fn test_loc() {
    let token = new_token();
    assert_eq!(token.loc(), &Loc::new(1, 2));
}

#[test]
fn test_lex_state_before() {
    let token = new_token();
    assert_eq!(token.lex_state_before(), lex_state(1));
}

#[test]
fn test_lex_state_after() {
    let token = new_token();
    assert_eq!(token.lex_state_after(), lex_state(2));
}
