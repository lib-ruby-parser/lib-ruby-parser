#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(trivial_casts, trivial_numeric_casts)]
#![warn(unused_qualifications)]
#![warn(deprecated_in_future)]
#![warn(unused_lifetimes)]

/*!
A Ruby parser written in Rust.

Uses bison under the hood.
*/

mod loc;

/// Module with everything related to output of the Parser, but not related to AST,
/// like `Comment`, `Input`, `CustomDecoder`
pub mod source;

mod lexer;

pub use lexer::Lexer;

mod static_environment;
pub use static_environment::StaticEnvironment;

pub(crate) mod parse_value;

mod parser_options;
pub use parser_options::ParserOptions;

mod parser_result;
pub use parser_result::ParserResult;

mod parser;
pub use parser::{token_name, Loc, Parser};

mod builder;
pub(crate) use builder::Builder;

mod current_arg_stack;
pub(crate) use current_arg_stack::CurrentArgStack;

mod max_numparam_stack;
pub(crate) use max_numparam_stack::MaxNumparamStack;

mod variables_stack;
pub(crate) use variables_stack::VariablesStack;

mod error;
pub use error::{Diagnostic, DiagnosticMessage, ErrorLevel};

pub(crate) mod maybe_byte;

mod lex_state;
pub use lex_state::lex_states;
pub use lex_state::LexState;

mod token_buf;
pub(crate) use token_buf::TokenBuf;

mod reserved_words;
pub use reserved_words::{reserved_word, ReservedWord};

mod stack_state;
pub(crate) use stack_state::StackState;

pub(crate) mod str_term;

mod context;
pub(crate) use context::{Context, ContextItem};

/// Module with all known node types
pub mod nodes;
pub use nodes::Node;

/// Module to perform recursive traversing
pub mod traverse;

mod string_value;
pub use string_value::StringValue;

mod token;
pub use token::Token;

/// Module to perform token rewriting
pub mod token_rewriter;

mod bytes;
pub use bytes::Bytes;

/// Debug level of the parser
pub mod debug_level;

/// Module with generic containers
pub mod containers;

/// Foreign parser options, can be casted to Rust ParserOptions
#[derive(Debug)]
#[repr(C)]
pub struct ForeignParserOptions {
    buffer_name: containers::List<u8>,
    debug: debug_level::Type,
    decoder: *const fn(
        input: containers::List<u8>,
        encoding: containers::List<u8>,
    ) -> containers::List<u8>,
    token_rewriter: *const fn() -> *mut u8,
    record_tokens: bool,
}

impl From<ForeignParserOptions> for ParserOptions {
    fn from(options: ForeignParserOptions) -> Self {
        let ForeignParserOptions {
            buffer_name, debug, ..
        } = options;

        use containers::maybe_ptr::MaybePtrNone;

        ParserOptions {
            buffer_name: String::from_utf8(buffer_name.into())
                .expect("Failed to convert buffer_name into UTF-8 string"),
            debug,
            decoder: containers::MaybePtr::none(),
            token_rewriter: None,
            record_tokens: true,
        }
    }
}

// /// Test
// #[no_mangle]
// pub extern "C" fn parse(
//     input: containers::List<u8>,
//     options: ForeignParserOptions,
// ) -> ParserResult {
//     let options = ParserOptions::from(options);
//     Parser::new(input, options).do_parse()
// }
