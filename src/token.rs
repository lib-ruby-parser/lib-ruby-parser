use core::{cell::Cell, ptr::NonNull};

use lib_ruby_parser_ast_arena::{Blob, Bytes, SingleLinkedIntrusiveListItem};

use crate::parser::token_name;
use crate::Loc;

/// A token that is emitted by a lexer and consumed by a parser
#[repr(C)]
pub struct Token<'b> {
    /// Numeric representation of the token type,
    /// e.g. 42 (for example) for tINTEGER
    pub token_type: i32,

    /// Value of the token,
    /// e.g "42" for 42
    pub token_value: &'b Bytes<'b>,

    /// Location of the token
    pub loc: Loc,

    next: Cell<Option<NonNull<Self>>>,

    marker: core::marker::PhantomData<&'b ()>,
}

impl<'b> Token<'b> {
    pub(crate) fn new(
        token_type: i32,
        token_value: &'b Bytes<'b>,
        loc: Loc,
        blob: &'b Blob<'b>,
    ) -> &'b Self {
        let this = blob.alloc_mut::<Self>();
        *this = Self {
            token_type,
            token_value,
            loc,
            next: Cell::new(None),
            marker: core::marker::PhantomData,
        };
        this
    }
    /// Returns a byte array of the token value
    // pub fn as_bytes(&self) -> &Vec<u8> {
    //     self.token_value.as_raw()
    // }

    /// Converts token value into `&str`
    // pub fn as_str_lossy(&self) -> Result<&str, core::str::Utf8Error> {
    //     core::str::from_utf8(self.token_value.as_raw())
    // }

    /// Converts token to a string, replaces unknown chars to `U+FFFD`
    pub fn to_string_lossy(&self) -> String {
        self.token_value.to_string_lossy()
    }

    /// Converts token to a string
    pub fn to_string(&self) -> Result<String, std::string::FromUtf8Error> {
        let bytes = self.token_value.iter().collect::<Vec<_>>();
        String::from_utf8(bytes)
    }

    /// Consumes a token and converts it into a string
    // pub fn into_string(self) -> Result<String, std::string::FromUtf8Error> {
    //     self.token_value.into_string()
    // }

    /// Returns name of the token
    pub fn token_name(&self) -> &'static str {
        token_name(self.token_type)
    }

    pub(crate) fn to_bytes_tmp(&self) -> lib_ruby_parser_ast::Bytes {
        let bytes = self.token_value.iter().collect::<Vec<_>>();
        lib_ruby_parser_ast::Bytes::new(bytes)
    }

    pub(crate) fn as_whole_str(&self) -> &'b str {
        self.token_value.as_whole_string().unwrap()
    }
}

impl core::fmt::Debug for Token<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{}, {:?}, {}...{}]",
            self.token_name(),
            self.token_value.to_string_lossy(),
            self.loc.begin,
            self.loc.end,
        )
    }
}

impl SingleLinkedIntrusiveListItem for Token<'_> {
    fn next(&self) -> Option<NonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: Option<NonNull<Self>>) {
        self.next.set(new_next)
    }
}

#[cfg(test)]
fn new_token<'b>(blob: &'b Blob<'b>) -> &'b Token<'b> {
    let bytes = blob.alloc_ref::<Bytes>();
    bytes.append_invalid_escaped(42, blob);
    Token::new(
        crate::Lexer::tINTEGER,
        bytes,
        Loc { begin: 1, end: 2 },
        blob,
    )
}

// #[test]
// fn test_as_bytes() {
//     let mut mem = vec![0; 1000];
//     let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());
//     let token = new_token(&blob);
//     assert_eq!(token.as_bytes(), &vec![42]);
// }

// #[test]
// fn test_as_str_lossy() {
//     let mut mem = vec![0; 1000];
//     let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());
//     let token = new_token(&blob);
//     assert_eq!(token.as_str_lossy(), Ok("*"));
// }

#[test]
fn test_to_string_lossy() {
    let mut mem = vec![0; 1000];
    let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());
    let token = new_token(&blob);
    assert_eq!(token.to_string_lossy(), String::from("*"));
}

#[test]
fn test_fmt() {
    let mut mem = vec![0; 1000];
    let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());
    let token = new_token(&blob);
    assert_eq!(format!("{:?}", token), "[tINTEGER, \"*\", 1...2]");
}
