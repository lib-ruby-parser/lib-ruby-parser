use crate::lexer::lex_states::*;
use crate::lexer::{Token, TokenType};

pub struct ReservedWord {
    pub name: &'static str,
    pub id: TokenType,
    pub modifier_id: TokenType,
    pub state: usize
}

// must be in sync with defs/keywords
pub const RESERVED_WORDS: [ReservedWord; 41] = [
    ReservedWord {
        name: "__ENCODING__",
        id: Token::k__ENCODING__,
        modifier_id: Token::k__ENCODING__,
        state: EXPR_END
    },
    ReservedWord {
        name: "__LINE__",
        id: Token::k__LINE__,
        modifier_id: Token::k__LINE__,
        state: EXPR_END
    },
    ReservedWord {
        name: "__FILE__",
        id: Token::k__FILE__,
        modifier_id: Token::k__FILE__,
        state: EXPR_END
    },
    ReservedWord {
        name: "BEGIN",
        id: Token::klBEGIN,
        modifier_id: Token::klBEGIN,
        state: EXPR_END
    },
    ReservedWord {
        name: "END",
        id: Token::klEND,
        modifier_id: Token::klEND,
        state: EXPR_END
    },
    ReservedWord {
        name: "alias",
        id: Token::kALIAS,
        modifier_id: Token::kALIAS,
        state: EXPR_FNAME|EXPR_FITEM
    },
    ReservedWord {
        name: "and",
        id: Token::kAND,
        modifier_id: Token::kAND,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "begin",
        id: Token::kBEGIN,
        modifier_id: Token::kBEGIN,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "break",
        id: Token::kBREAK,
        modifier_id: Token::kBREAK,
        state: EXPR_MID
    },
    ReservedWord {
        name: "case",
        id: Token::kCASE,
        modifier_id: Token::kCASE,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "class",
        id: Token::kCLASS,
        modifier_id: Token::kCLASS,
        state: EXPR_CLASS
    },
    ReservedWord {
        name: "def",
        id: Token::kDEF,
        modifier_id: Token::kDEF,
        state: EXPR_FNAME
    },
    ReservedWord {
        name: "defined?",
        id: Token::kDEFINED,
        modifier_id: Token::kDEFINED,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "do",
        id: Token::kDO,
        modifier_id: Token::kDO,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "else",
        id: Token::kELSE,
        modifier_id: Token::kELSE,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "elsif",
        id: Token::kELSIF,
        modifier_id: Token::kELSIF,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "end",
        id: Token::kEND,
        modifier_id: Token::kEND,
        state: EXPR_END
    },
    ReservedWord {
        name: "ensure",
        id: Token::kENSURE,
        modifier_id: Token::kENSURE,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "false",
        id: Token::kFALSE,
        modifier_id: Token::kFALSE,
        state: EXPR_END
    },
    ReservedWord {
        name: "for",
        id: Token::kFOR,
        modifier_id: Token::kFOR,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "if",
        id: Token::kIF,
        modifier_id: Token::kIF_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "in",
        id: Token::kIN,
        modifier_id: Token::kIN,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "module",
        id: Token::kMODULE,
        modifier_id: Token::kMODULE,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "next",
        id: Token::kNEXT,
        modifier_id: Token::kNEXT,
        state: EXPR_MID
    },
    ReservedWord {
        name: "nil",
        id: Token::kNIL,
        modifier_id: Token::kNIL,
        state: EXPR_END
    },
    ReservedWord {
        name: "not",
        id: Token::kNOT,
        modifier_id: Token::kNOT,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "or",
        id: Token::kOR,
        modifier_id: Token::kOR,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "redo",
        id: Token::kREDO,
        modifier_id: Token::kREDO,
        state: EXPR_END
    },
    ReservedWord {
        name: "rescue",
        id: Token::kRESCUE,
        modifier_id: Token::kRESCUE_MOD,
        state: EXPR_MID
    },
    ReservedWord {
        name: "retry",
        id: Token::kRETRY,
        modifier_id: Token::kRETRY,
        state: EXPR_END
    },
    ReservedWord {
        name: "return",
        id: Token::kRETURN,
        modifier_id: Token::kRETURN,
        state: EXPR_MID
    },
    ReservedWord {
        name: "self",
        id: Token::kSELF,
        modifier_id: Token::kSELF,
        state: EXPR_END
    },
    ReservedWord {
        name: "super",
        id: Token::kSUPER,
        modifier_id: Token::kSUPER,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "then",
        id: Token::kTHEN,
        modifier_id: Token::kTHEN,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "true",
        id: Token::kTRUE,
        modifier_id: Token::kTRUE,
        state: EXPR_END
    },
    ReservedWord {
        name: "undef",
        id: Token::kUNDEF,
        modifier_id: Token::kUNDEF,
        state: EXPR_FNAME|EXPR_FITEM
    },
    ReservedWord {
        name: "unless",
        id: Token::kUNLESS,
        modifier_id: Token::kUNLESS_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "until",
        id: Token::kUNTIL,
        modifier_id: Token::kUNTIL_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "when",
        id: Token::kWHEN,
        modifier_id: Token::kWHEN,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "while",
        id: Token::kWHILE,
        modifier_id: Token::kWHILE_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "yield",
        id: Token::kYIELD,
        modifier_id: Token::kYIELD,
        state: EXPR_ARG
    },
];

pub fn reserved_word(tok: &str) -> Option<&'static ReservedWord> {
    for res in RESERVED_WORDS.iter() {
        if res.name == tok {
            return Some(res)
        }
    }
    None
}
