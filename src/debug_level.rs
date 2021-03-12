/// A type of the debug level
pub type Type = i8;

/// Print no debug information
pub const NONE: Type = 0;

/// Print only debug information of the parser
pub const PARSER: Type = 1 << 0;

/// Print only debug information of the lexer
pub const LEXER: Type = 1 << 1;

/// Print only debug information of the Buffer
pub const BUFFER: Type = 1 << 2;

/// Print everything
pub const ALL: Type = PARSER | LEXER | BUFFER;

pub(crate) fn is_debug_parser(debug_level: Type) -> bool {
    (debug_level & PARSER) != 0
}

pub(crate) fn is_debug_lexer(debug_level: Type) -> bool {
    (debug_level & LEXER) != 0
}

pub(crate) fn is_debug_buffer(debug_level: Type) -> bool {
    (debug_level & BUFFER) != 0
}
