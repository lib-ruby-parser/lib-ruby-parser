mod parse;
use lib_ruby_parser_ast::Loc;
pub use parse::{token_name, Parser};

#[cfg(feature = "debug-parser")]
macro_rules! println_if_debug_parser {
    () => {
        println!("");
    };
    ($($arg:tt)*) => {{
        use std::io::Write;
        let mut stderr = std::io::stderr();
        stderr.write_fmt(core::format_args!($($arg)*)).unwrap();
        stderr.write(b"\n").unwrap();
    }};
}
#[cfg(not(feature = "debug-parser"))]
macro_rules! println_if_debug_parser {
    () => {};
    ($($arg:tt)*) => {{}};
}
pub(crate) use println_if_debug_parser;

use crate::parse_value::ParseValue;

#[derive(Debug)]
pub(crate) struct YYStackItem<'b> {
    state: i32,
    value: ParseValue<'b>,
    loc: Loc,
}

#[derive(Debug)]
pub(crate) struct YYStack<'b> {
    stack: Vec<YYStackItem<'b>>,
}

impl<'b> YYStack<'b> {
    pub(crate) fn new() -> Self {
        Self {
            stack: Vec::with_capacity(20),
        }
    }

    pub(crate) fn push(&mut self, state: i32, value: ParseValue<'b>, loc: Loc) {
        self.stack.push(YYStackItem { state, value, loc });
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn pop_n(&mut self, num: usize) {
        let len = self.stack.len() - num;
        self.stack.truncate(len);
    }

    pub(crate) fn state_at(&self, i: usize) -> i32 {
        self.stack[self.len() - 1 - i].state
    }

    pub(crate) fn location_at(&self, i: usize) -> Loc {
        self.stack[self.len() - 1 - i].loc
    }

    pub(crate) fn borrow_value_at(&self, i: usize) -> ParseValue<'b> {
        self.stack[self.len() - 1 - i].value
    }

    pub(crate) fn owned_value_at(&mut self, i: usize) -> ParseValue<'b> {
        let len = self.len();
        self.stack[len - 1 - i].value
    }

    pub(crate) fn len(&self) -> usize {
        self.stack.len()
    }
}
