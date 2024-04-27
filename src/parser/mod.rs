mod parse;
use lib_ruby_parser_ast::{Loc, Usized};
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

const PARSE_VALUE_IN_WORDS: usize =
    core::mem::size_of::<ParseValue>() / core::mem::size_of::<usize>();
impl Usized<PARSE_VALUE_IN_WORDS> for ParseValue<'_> {}

/// A stack item
#[derive(Debug, Clone, Copy)]
pub struct YYStackItem {
    state: i32,
    value: [usize; PARSE_VALUE_IN_WORDS],
    loc: Loc,
}

impl YYStackItem {
    /// Constructs an empty stack item
    pub fn none() -> Self {
        Self {
            state: 0,
            value: ParseValue::None.to_words(),
            loc: Loc::new(0, 0),
        }
    }
}

#[derive(Debug)]
pub struct YYStack<'s> {
    mem: &'s mut [YYStackItem],
    len: usize,
}

impl<'s> YYStack<'s> {
    pub(crate) fn new(mem: &'s mut [YYStackItem]) -> Self {
        Self { mem, len: 0 }
    }

    pub(crate) fn push(&mut self, state: i32, value: ParseValue<'_>, loc: Loc) {
        self.mem[self.len] = YYStackItem {
            state,
            value: value.to_words(),
            loc,
        };
        self.len += 1;
    }

    pub(crate) fn pop(&mut self) {
        self.len -= 1;
    }

    pub(crate) fn pop_n(&mut self, num: usize) {
        self.len -= num;
    }

    pub(crate) fn state_at(&self, i: usize) -> i32 {
        self.mem[self.len - 1 - i].state
    }

    pub(crate) fn location_at(&self, i: usize) -> Loc {
        self.mem[self.len - 1 - i].loc
    }

    pub(crate) fn borrow_value_at<'b>(&self, i: usize) -> ParseValue<'b> {
        ParseValue::from_words(self.mem[self.len - 1 - i].value)
    }

    pub(crate) fn owned_value_at<'b>(&mut self, i: usize) -> ParseValue<'b> {
        ParseValue::from_words(self.mem[self.len - 1 - i].value)
    }

    pub(crate) fn len(&self) -> usize {
        self.len
    }
}

impl core::fmt::Display for YYStack<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Stack now states =")?;
        for item in self.mem.iter() {
            write!(f, " {}", item.state)?;
        }
        write!(f, " / values =")?;
        for item in self.mem.iter() {
            write!(f, " {:?}", item.value)?;
        }
        Ok(())
    }
}
