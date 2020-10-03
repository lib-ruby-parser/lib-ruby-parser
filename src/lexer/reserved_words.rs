use crate::lexer::lex_states::*;
use crate::Lexer;

pub struct ReservedWord {
    pub name: Vec<u8>,
    pub id: i32,
    pub modifier_id: i32,
    pub state: i32
}

lazy_static! {
    // must be in sync with defs/keywords
    pub static ref RESERVED_WORDS: Vec<ReservedWord> = vec![
        ReservedWord {
            name: b"__ENCODING__".to_vec(),
            id: Lexer::k__ENCODING__,
            modifier_id: Lexer::k__ENCODING__,
            state: EXPR_END
        },
        ReservedWord {
            name: b"__LINE__".to_vec(),
            id: Lexer::k__LINE__,
            modifier_id: Lexer::k__LINE__,
            state: EXPR_END
        },
        ReservedWord {
            name: b"__FILE__".to_vec(),
            id: Lexer::k__FILE__,
            modifier_id: Lexer::k__FILE__,
            state: EXPR_END
        },
        ReservedWord {
            name: b"BEGIN".to_vec(),
            id: Lexer::klBEGIN,
            modifier_id: Lexer::klBEGIN,
            state: EXPR_END
        },
        ReservedWord {
            name: b"END".to_vec(),
            id: Lexer::klEND,
            modifier_id: Lexer::klEND,
            state: EXPR_END
        },
        ReservedWord {
            name: b"alias".to_vec(),
            id: Lexer::kALIAS,
            modifier_id: Lexer::kALIAS,
            state: EXPR_FNAME|EXPR_FITEM
        },
        ReservedWord {
            name: b"and".to_vec(),
            id: Lexer::kAND,
            modifier_id: Lexer::kAND,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"begin".to_vec(),
            id: Lexer::kBEGIN,
            modifier_id: Lexer::kBEGIN,
            state: EXPR_BEG
        },
        ReservedWord {
            name: b"break".to_vec(),
            id: Lexer::kBREAK,
            modifier_id: Lexer::kBREAK,
            state: EXPR_MID
        },
        ReservedWord {
            name: b"case".to_vec(),
            id: Lexer::kCASE,
            modifier_id: Lexer::kCASE,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"class".to_vec(),
            id: Lexer::kCLASS,
            modifier_id: Lexer::kCLASS,
            state: EXPR_CLASS
        },
        ReservedWord {
            name: b"def".to_vec(),
            id: Lexer::kDEF,
            modifier_id: Lexer::kDEF,
            state: EXPR_FNAME
        },
        ReservedWord {
            name: b"defined?".to_vec(),
            id: Lexer::kDEFINED,
            modifier_id: Lexer::kDEFINED,
            state: EXPR_ARG
        },
        ReservedWord {
            name: b"do".to_vec(),
            id: Lexer::kDO,
            modifier_id: Lexer::kDO,
            state: EXPR_BEG
        },
        ReservedWord {
            name: b"else".to_vec(),
            id: Lexer::kELSE,
            modifier_id: Lexer::kELSE,
            state: EXPR_BEG
        },
        ReservedWord {
            name: b"elsif".to_vec(),
            id: Lexer::kELSIF,
            modifier_id: Lexer::kELSIF,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"end".to_vec(),
            id: Lexer::kEND,
            modifier_id: Lexer::kEND,
            state: EXPR_END
        },
        ReservedWord {
            name: b"ensure".to_vec(),
            id: Lexer::kENSURE,
            modifier_id: Lexer::kENSURE,
            state: EXPR_BEG
        },
        ReservedWord {
            name: b"false".to_vec(),
            id: Lexer::kFALSE,
            modifier_id: Lexer::kFALSE,
            state: EXPR_END
        },
        ReservedWord {
            name: b"for".to_vec(),
            id: Lexer::kFOR,
            modifier_id: Lexer::kFOR,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"if".to_vec(),
            id: Lexer::kIF,
            modifier_id: Lexer::kIF_MOD,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"in".to_vec(),
            id: Lexer::kIN,
            modifier_id: Lexer::kIN,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"module".to_vec(),
            id: Lexer::kMODULE,
            modifier_id: Lexer::kMODULE,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"next".to_vec(),
            id: Lexer::kNEXT,
            modifier_id: Lexer::kNEXT,
            state: EXPR_MID
        },
        ReservedWord {
            name: b"nil".to_vec(),
            id: Lexer::kNIL,
            modifier_id: Lexer::kNIL,
            state: EXPR_END
        },
        ReservedWord {
            name: b"not".to_vec(),
            id: Lexer::kNOT,
            modifier_id: Lexer::kNOT,
            state: EXPR_ARG
        },
        ReservedWord {
            name: b"or".to_vec(),
            id: Lexer::kOR,
            modifier_id: Lexer::kOR,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"redo".to_vec(),
            id: Lexer::kREDO,
            modifier_id: Lexer::kREDO,
            state: EXPR_END
        },
        ReservedWord {
            name: b"rescue".to_vec(),
            id: Lexer::kRESCUE,
            modifier_id: Lexer::kRESCUE_MOD,
            state: EXPR_MID
        },
        ReservedWord {
            name: b"retry".to_vec(),
            id: Lexer::kRETRY,
            modifier_id: Lexer::kRETRY,
            state: EXPR_END
        },
        ReservedWord {
            name: b"return".to_vec(),
            id: Lexer::kRETURN,
            modifier_id: Lexer::kRETURN,
            state: EXPR_MID
        },
        ReservedWord {
            name: b"self".to_vec(),
            id: Lexer::kSELF,
            modifier_id: Lexer::kSELF,
            state: EXPR_END
        },
        ReservedWord {
            name: b"super".to_vec(),
            id: Lexer::kSUPER,
            modifier_id: Lexer::kSUPER,
            state: EXPR_ARG
        },
        ReservedWord {
            name: b"then".to_vec(),
            id: Lexer::kTHEN,
            modifier_id: Lexer::kTHEN,
            state: EXPR_BEG
        },
        ReservedWord {
            name: b"true".to_vec(),
            id: Lexer::kTRUE,
            modifier_id: Lexer::kTRUE,
            state: EXPR_END
        },
        ReservedWord {
            name: b"undef".to_vec(),
            id: Lexer::kUNDEF,
            modifier_id: Lexer::kUNDEF,
            state: EXPR_FNAME|EXPR_FITEM
        },
        ReservedWord {
            name: b"unless".to_vec(),
            id: Lexer::kUNLESS,
            modifier_id: Lexer::kUNLESS_MOD,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"until".to_vec(),
            id: Lexer::kUNTIL,
            modifier_id: Lexer::kUNTIL_MOD,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"when".to_vec(),
            id: Lexer::kWHEN,
            modifier_id: Lexer::kWHEN,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"while".to_vec(),
            id: Lexer::kWHILE,
            modifier_id: Lexer::kWHILE_MOD,
            state: EXPR_VALUE
        },
        ReservedWord {
            name: b"yield".to_vec(),
            id: Lexer::kYIELD,
            modifier_id: Lexer::kYIELD,
            state: EXPR_ARG
        },
    ];
}

pub fn reserved_word(tok: &Vec<u8>) -> Option<&'static ReservedWord> {
    for res in RESERVED_WORDS.iter() {
        if &res.name == tok {
            return Some(res)
        }
    }
    None
}
