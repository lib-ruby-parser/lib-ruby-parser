use crate::lexer::lex_states::*;
use crate::lexer::TokenType;

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
        id: TokenType::k__ENCODING__,
        modifier_id: TokenType::k__ENCODING__,
        state: EXPR_END
    },
    ReservedWord {
        name: "__LINE__",
        id: TokenType::k__LINE__,
        modifier_id: TokenType::k__LINE__,
        state: EXPR_END
    },
    ReservedWord {
        name: "__FILE__",
        id: TokenType::k__FILE__,
        modifier_id: TokenType::k__FILE__,
        state: EXPR_END
    },
    ReservedWord {
        name: "BEGIN",
        id: TokenType::klBEGIN,
        modifier_id: TokenType::klBEGIN,
        state: EXPR_END
    },
    ReservedWord {
        name: "END",
        id: TokenType::klEND,
        modifier_id: TokenType::klEND,
        state: EXPR_END
    },
    ReservedWord {
        name: "alias",
        id: TokenType::kALIAS,
        modifier_id: TokenType::kALIAS,
        state: EXPR_FNAME|EXPR_FITEM
    },
    ReservedWord {
        name: "and",
        id: TokenType::kAND,
        modifier_id: TokenType::kAND,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "begin",
        id: TokenType::kBEGIN,
        modifier_id: TokenType::kBEGIN,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "break",
        id: TokenType::kBREAK,
        modifier_id: TokenType::kBREAK,
        state: EXPR_MID
    },
    ReservedWord {
        name: "case",
        id: TokenType::kCASE,
        modifier_id: TokenType::kCASE,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "class",
        id: TokenType::kCLASS,
        modifier_id: TokenType::kCLASS,
        state: EXPR_CLASS
    },
    ReservedWord {
        name: "def",
        id: TokenType::kDEF,
        modifier_id: TokenType::kDEF,
        state: EXPR_FNAME
    },
    ReservedWord {
        name: "defined?",
        id: TokenType::kDEFINED,
        modifier_id: TokenType::kDEFINED,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "do",
        id: TokenType::kDO,
        modifier_id: TokenType::kDO,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "else",
        id: TokenType::kELSE,
        modifier_id: TokenType::kELSE,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "elsif",
        id: TokenType::kELSIF,
        modifier_id: TokenType::kELSIF,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "end",
        id: TokenType::kEND,
        modifier_id: TokenType::kEND,
        state: EXPR_END
    },
    ReservedWord {
        name: "ensure",
        id: TokenType::kENSURE,
        modifier_id: TokenType::kENSURE,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "false",
        id: TokenType::kFALSE,
        modifier_id: TokenType::kFALSE,
        state: EXPR_END
    },
    ReservedWord {
        name: "for",
        id: TokenType::kFOR,
        modifier_id: TokenType::kFOR,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "if",
        id: TokenType::kIF,
        modifier_id: TokenType::kIF_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "in",
        id: TokenType::kIN,
        modifier_id: TokenType::kIN,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "module",
        id: TokenType::kMODULE,
        modifier_id: TokenType::kMODULE,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "next",
        id: TokenType::kNEXT,
        modifier_id: TokenType::kNEXT,
        state: EXPR_MID
    },
    ReservedWord {
        name: "nil",
        id: TokenType::kNIL,
        modifier_id: TokenType::kNIL,
        state: EXPR_END
    },
    ReservedWord {
        name: "not",
        id: TokenType::kNOT,
        modifier_id: TokenType::kNOT,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "or",
        id: TokenType::kOR,
        modifier_id: TokenType::kOR,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "redo",
        id: TokenType::kREDO,
        modifier_id: TokenType::kREDO,
        state: EXPR_END
    },
    ReservedWord {
        name: "rescue",
        id: TokenType::kRESCUE,
        modifier_id: TokenType::kRESCUE_MOD,
        state: EXPR_MID
    },
    ReservedWord {
        name: "retry",
        id: TokenType::kRETRY,
        modifier_id: TokenType::kRETRY,
        state: EXPR_END
    },
    ReservedWord {
        name: "return",
        id: TokenType::kRETURN,
        modifier_id: TokenType::kRETURN,
        state: EXPR_MID
    },
    ReservedWord {
        name: "self",
        id: TokenType::kSELF,
        modifier_id: TokenType::kSELF,
        state: EXPR_END
    },
    ReservedWord {
        name: "super",
        id: TokenType::kSUPER,
        modifier_id: TokenType::kSUPER,
        state: EXPR_ARG
    },
    ReservedWord {
        name: "then",
        id: TokenType::kTHEN,
        modifier_id: TokenType::kTHEN,
        state: EXPR_BEG
    },
    ReservedWord {
        name: "true",
        id: TokenType::kTRUE,
        modifier_id: TokenType::kTRUE,
        state: EXPR_END
    },
    ReservedWord {
        name: "undef",
        id: TokenType::kUNDEF,
        modifier_id: TokenType::kUNDEF,
        state: EXPR_FNAME|EXPR_FITEM
    },
    ReservedWord {
        name: "unless",
        id: TokenType::kUNLESS,
        modifier_id: TokenType::kUNLESS_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "until",
        id: TokenType::kUNTIL,
        modifier_id: TokenType::kUNTIL_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "when",
        id: TokenType::kWHEN,
        modifier_id: TokenType::kWHEN,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "while",
        id: TokenType::kWHILE,
        modifier_id: TokenType::kWHILE_MOD,
        state: EXPR_VALUE
    },
    ReservedWord {
        name: "yield",
        id: TokenType::kYIELD,
        modifier_id: TokenType::kYIELD,
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
