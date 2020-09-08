use std::rc::Rc;

use crate::source::buffer::Buffer;
use crate::lexer::{StackState};

enum TokenType {}

struct Token {
    _type_: TokenType,
    _value: TokenValue
}

struct TokenValue {
    _value: String,
    _range: std::ops::Range<usize>
}

struct Comment {}


struct Context {}

pub struct Lexer {
    _buffer: Buffer,

    // diagnostics: Rc<dyn Diagnostics>,
    // static_env: Rc<StaticEnv>,

    _cond: StackState,
    _cmdarg: StackState,
    _in_kwarg: bool,
    _context: Rc<Context>,
    _command_start: bool,

    _tokens: Vec<Token>,
    _comments: Vec<Comment>
}
