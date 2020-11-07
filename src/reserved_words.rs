use crate::lex_states::*;
use crate::Lexer;

pub struct ReservedWord {
    pub(crate) name: &'static str,
    pub(crate) id: i32,
    pub(crate) modifier_id: i32,
    pub(crate) state: i32,
}

// must be in sync with defs/keywords
const RESERVED_WORDS: &[ReservedWord] = &[
    ReservedWord {
        name: "BEGIN",
        id: Lexer::klBEGIN,
        modifier_id: Lexer::klBEGIN,
        state: EXPR_END,
    },
    ReservedWord {
        name: "END",
        id: Lexer::klEND,
        modifier_id: Lexer::klEND,
        state: EXPR_END,
    },
    ReservedWord {
        name: "__ENCODING__",
        id: Lexer::k__ENCODING__,
        modifier_id: Lexer::k__ENCODING__,
        state: EXPR_END,
    },
    ReservedWord {
        name: "__FILE__",
        id: Lexer::k__FILE__,
        modifier_id: Lexer::k__FILE__,
        state: EXPR_END,
    },
    ReservedWord {
        name: "__LINE__",
        id: Lexer::k__LINE__,
        modifier_id: Lexer::k__LINE__,
        state: EXPR_END,
    },
    ReservedWord {
        name: "alias",
        id: Lexer::kALIAS,
        modifier_id: Lexer::kALIAS,
        state: EXPR_FNAME | EXPR_FITEM,
    },
    ReservedWord {
        name: "and",
        id: Lexer::kAND,
        modifier_id: Lexer::kAND,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "begin",
        id: Lexer::kBEGIN,
        modifier_id: Lexer::kBEGIN,
        state: EXPR_BEG,
    },
    ReservedWord {
        name: "break",
        id: Lexer::kBREAK,
        modifier_id: Lexer::kBREAK,
        state: EXPR_MID,
    },
    ReservedWord {
        name: "case",
        id: Lexer::kCASE,
        modifier_id: Lexer::kCASE,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "class",
        id: Lexer::kCLASS,
        modifier_id: Lexer::kCLASS,
        state: EXPR_CLASS,
    },
    ReservedWord {
        name: "def",
        id: Lexer::kDEF,
        modifier_id: Lexer::kDEF,
        state: EXPR_FNAME,
    },
    ReservedWord {
        name: "defined?",
        id: Lexer::kDEFINED,
        modifier_id: Lexer::kDEFINED,
        state: EXPR_ARG,
    },
    ReservedWord {
        name: "do",
        id: Lexer::kDO,
        modifier_id: Lexer::kDO,
        state: EXPR_BEG,
    },
    ReservedWord {
        name: "else",
        id: Lexer::kELSE,
        modifier_id: Lexer::kELSE,
        state: EXPR_BEG,
    },
    ReservedWord {
        name: "elsif",
        id: Lexer::kELSIF,
        modifier_id: Lexer::kELSIF,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "end",
        id: Lexer::kEND,
        modifier_id: Lexer::kEND,
        state: EXPR_END,
    },
    ReservedWord {
        name: "ensure",
        id: Lexer::kENSURE,
        modifier_id: Lexer::kENSURE,
        state: EXPR_BEG,
    },
    ReservedWord {
        name: "false",
        id: Lexer::kFALSE,
        modifier_id: Lexer::kFALSE,
        state: EXPR_END,
    },
    ReservedWord {
        name: "for",
        id: Lexer::kFOR,
        modifier_id: Lexer::kFOR,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "if",
        id: Lexer::kIF,
        modifier_id: Lexer::kIF_MOD,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "in",
        id: Lexer::kIN,
        modifier_id: Lexer::kIN,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "module",
        id: Lexer::kMODULE,
        modifier_id: Lexer::kMODULE,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "next",
        id: Lexer::kNEXT,
        modifier_id: Lexer::kNEXT,
        state: EXPR_MID,
    },
    ReservedWord {
        name: "nil",
        id: Lexer::kNIL,
        modifier_id: Lexer::kNIL,
        state: EXPR_END,
    },
    ReservedWord {
        name: "not",
        id: Lexer::kNOT,
        modifier_id: Lexer::kNOT,
        state: EXPR_ARG,
    },
    ReservedWord {
        name: "or",
        id: Lexer::kOR,
        modifier_id: Lexer::kOR,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "redo",
        id: Lexer::kREDO,
        modifier_id: Lexer::kREDO,
        state: EXPR_END,
    },
    ReservedWord {
        name: "rescue",
        id: Lexer::kRESCUE,
        modifier_id: Lexer::kRESCUE_MOD,
        state: EXPR_MID,
    },
    ReservedWord {
        name: "retry",
        id: Lexer::kRETRY,
        modifier_id: Lexer::kRETRY,
        state: EXPR_END,
    },
    ReservedWord {
        name: "return",
        id: Lexer::kRETURN,
        modifier_id: Lexer::kRETURN,
        state: EXPR_MID,
    },
    ReservedWord {
        name: "self",
        id: Lexer::kSELF,
        modifier_id: Lexer::kSELF,
        state: EXPR_END,
    },
    ReservedWord {
        name: "super",
        id: Lexer::kSUPER,
        modifier_id: Lexer::kSUPER,
        state: EXPR_ARG,
    },
    ReservedWord {
        name: "then",
        id: Lexer::kTHEN,
        modifier_id: Lexer::kTHEN,
        state: EXPR_BEG,
    },
    ReservedWord {
        name: "true",
        id: Lexer::kTRUE,
        modifier_id: Lexer::kTRUE,
        state: EXPR_END,
    },
    ReservedWord {
        name: "undef",
        id: Lexer::kUNDEF,
        modifier_id: Lexer::kUNDEF,
        state: EXPR_FNAME | EXPR_FITEM,
    },
    ReservedWord {
        name: "unless",
        id: Lexer::kUNLESS,
        modifier_id: Lexer::kUNLESS_MOD,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "until",
        id: Lexer::kUNTIL,
        modifier_id: Lexer::kUNTIL_MOD,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "when",
        id: Lexer::kWHEN,
        modifier_id: Lexer::kWHEN,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "while",
        id: Lexer::kWHILE,
        modifier_id: Lexer::kWHILE_MOD,
        state: EXPR_VALUE,
    },
    ReservedWord {
        name: "yield",
        id: Lexer::kYIELD,
        modifier_id: Lexer::kYIELD,
        state: EXPR_ARG,
    },
];

pub(crate) fn reserved_word(tok: &str) -> Option<&'static ReservedWord> {
    debug_assert!(
        RESERVED_WORDS.is_sorted_by(|one, two| Some(one.name.cmp(two.name))),
        "\nRESERVED_WORDS must be sorted. Expected:\n{:?}\nGot:\n{:?}\n",
        {
            let mut words = RESERVED_WORDS.iter().map(|w| w.name).collect::<Vec<_>>();
            words.sort();
            words
        },
        RESERVED_WORDS.iter().map(|w| w.name).collect::<Vec<_>>()
    );

    let idx = RESERVED_WORDS.binary_search_by(|e| e.name.cmp(tok)).ok()?;
    Some(&RESERVED_WORDS[idx])
}
