use std::collections::HashMap;

#[derive(Clone)]
struct RawString {
    s: &'static str,
}

#[derive(Clone)]
struct ReservedWord {
    name: &'static str,
    id: RawString,
    modifier_id: RawString,
    state: RawString,
}

const RESERVED_WORDS: &[ReservedWord] = &[
    ReservedWord {
        name: "BEGIN",
        id: RawString {
            s: "Lexer::klBEGIN",
        },
        modifier_id: RawString {
            s: "Lexer::klBEGIN",
        },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "END",
        id: RawString { s: "Lexer::klEND" },
        modifier_id: RawString { s: "Lexer::klEND" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "__ENCODING__",
        id: RawString {
            s: "Lexer::k__ENCODING__",
        },
        modifier_id: RawString {
            s: "Lexer::k__ENCODING__",
        },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "__FILE__",
        id: RawString {
            s: "Lexer::k__FILE__",
        },
        modifier_id: RawString {
            s: "Lexer::k__FILE__",
        },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "__LINE__",
        id: RawString {
            s: "Lexer::k__LINE__",
        },
        modifier_id: RawString {
            s: "Lexer::k__LINE__",
        },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "alias",
        id: RawString { s: "Lexer::kALIAS" },
        modifier_id: RawString { s: "Lexer::kALIAS" },
        state: RawString {
            s: "EXPR_FNAME | EXPR_FITEM",
        },
    },
    ReservedWord {
        name: "and",
        id: RawString { s: "Lexer::kAND" },
        modifier_id: RawString { s: "Lexer::kAND" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "begin",
        id: RawString { s: "Lexer::kBEGIN" },
        modifier_id: RawString { s: "Lexer::kBEGIN" },
        state: RawString { s: "EXPR_BEG" },
    },
    ReservedWord {
        name: "break",
        id: RawString { s: "Lexer::kBREAK" },
        modifier_id: RawString { s: "Lexer::kBREAK" },
        state: RawString { s: "EXPR_MID" },
    },
    ReservedWord {
        name: "case",
        id: RawString { s: "Lexer::kCASE" },
        modifier_id: RawString { s: "Lexer::kCASE" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "class",
        id: RawString { s: "Lexer::kCLASS" },
        modifier_id: RawString { s: "Lexer::kCLASS" },
        state: RawString { s: "EXPR_CLASS" },
    },
    ReservedWord {
        name: "def",
        id: RawString { s: "Lexer::kDEF" },
        modifier_id: RawString { s: "Lexer::kDEF" },
        state: RawString { s: "EXPR_FNAME" },
    },
    ReservedWord {
        name: "defined?",
        id: RawString {
            s: "Lexer::kDEFINED",
        },
        modifier_id: RawString {
            s: "Lexer::kDEFINED",
        },
        state: RawString { s: "EXPR_ARG" },
    },
    ReservedWord {
        name: "do",
        id: RawString { s: "Lexer::kDO" },
        modifier_id: RawString { s: "Lexer::kDO" },
        state: RawString { s: "EXPR_BEG" },
    },
    ReservedWord {
        name: "else",
        id: RawString { s: "Lexer::kELSE" },
        modifier_id: RawString { s: "Lexer::kELSE" },
        state: RawString { s: "EXPR_BEG" },
    },
    ReservedWord {
        name: "elsif",
        id: RawString { s: "Lexer::kELSIF" },
        modifier_id: RawString { s: "Lexer::kELSIF" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "end",
        id: RawString { s: "Lexer::kEND" },
        modifier_id: RawString { s: "Lexer::kEND" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "ensure",
        id: RawString {
            s: "Lexer::kENSURE",
        },
        modifier_id: RawString {
            s: "Lexer::kENSURE",
        },
        state: RawString { s: "EXPR_BEG" },
    },
    ReservedWord {
        name: "false",
        id: RawString { s: "Lexer::kFALSE" },
        modifier_id: RawString { s: "Lexer::kFALSE" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "for",
        id: RawString { s: "Lexer::kFOR" },
        modifier_id: RawString { s: "Lexer::kFOR" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "if",
        id: RawString { s: "Lexer::kIF" },
        modifier_id: RawString {
            s: "Lexer::kIF_MOD",
        },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "in",
        id: RawString { s: "Lexer::kIN" },
        modifier_id: RawString { s: "Lexer::kIN" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "module",
        id: RawString {
            s: "Lexer::kMODULE",
        },
        modifier_id: RawString {
            s: "Lexer::kMODULE",
        },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "next",
        id: RawString { s: "Lexer::kNEXT" },
        modifier_id: RawString { s: "Lexer::kNEXT" },
        state: RawString { s: "EXPR_MID" },
    },
    ReservedWord {
        name: "nil",
        id: RawString { s: "Lexer::kNIL" },
        modifier_id: RawString { s: "Lexer::kNIL" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "not",
        id: RawString { s: "Lexer::kNOT" },
        modifier_id: RawString { s: "Lexer::kNOT" },
        state: RawString { s: "EXPR_ARG" },
    },
    ReservedWord {
        name: "or",
        id: RawString { s: "Lexer::kOR" },
        modifier_id: RawString { s: "Lexer::kOR" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "redo",
        id: RawString { s: "Lexer::kREDO" },
        modifier_id: RawString { s: "Lexer::kREDO" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "rescue",
        id: RawString {
            s: "Lexer::kRESCUE",
        },
        modifier_id: RawString {
            s: "Lexer::kRESCUE_MOD",
        },
        state: RawString { s: "EXPR_MID" },
    },
    ReservedWord {
        name: "retry",
        id: RawString { s: "Lexer::kRETRY" },
        modifier_id: RawString { s: "Lexer::kRETRY" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "return",
        id: RawString {
            s: "Lexer::kRETURN",
        },
        modifier_id: RawString {
            s: "Lexer::kRETURN",
        },
        state: RawString { s: "EXPR_MID" },
    },
    ReservedWord {
        name: "self",
        id: RawString { s: "Lexer::kSELF" },
        modifier_id: RawString { s: "Lexer::kSELF" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "super",
        id: RawString { s: "Lexer::kSUPER" },
        modifier_id: RawString { s: "Lexer::kSUPER" },
        state: RawString { s: "EXPR_ARG" },
    },
    ReservedWord {
        name: "then",
        id: RawString { s: "Lexer::kTHEN" },
        modifier_id: RawString { s: "Lexer::kTHEN" },
        state: RawString { s: "EXPR_BEG" },
    },
    ReservedWord {
        name: "true",
        id: RawString { s: "Lexer::kTRUE" },
        modifier_id: RawString { s: "Lexer::kTRUE" },
        state: RawString { s: "EXPR_END" },
    },
    ReservedWord {
        name: "undef",
        id: RawString { s: "Lexer::kUNDEF" },
        modifier_id: RawString { s: "Lexer::kUNDEF" },
        state: RawString {
            s: "EXPR_FNAME | EXPR_FITEM",
        },
    },
    ReservedWord {
        name: "unless",
        id: RawString {
            s: "Lexer::kUNLESS",
        },
        modifier_id: RawString {
            s: "Lexer::kUNLESS_MOD",
        },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "until",
        id: RawString { s: "Lexer::kUNTIL" },
        modifier_id: RawString {
            s: "Lexer::kUNTIL_MOD",
        },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "when",
        id: RawString { s: "Lexer::kWHEN" },
        modifier_id: RawString { s: "Lexer::kWHEN" },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "while",
        id: RawString { s: "Lexer::kWHILE" },
        modifier_id: RawString {
            s: "Lexer::kWHILE_MOD",
        },
        state: RawString { s: "EXPR_VALUE" },
    },
    ReservedWord {
        name: "yield",
        id: RawString { s: "Lexer::kYIELD" },
        modifier_id: RawString { s: "Lexer::kYIELD" },
        state: RawString { s: "EXPR_ARG" },
    },
];

pub(crate) struct ReservedWordsList {
    inner: HashMap<usize, Vec<ReservedWord>>,
}

impl ReservedWordsList {
    pub(crate) fn new() -> Self {
        let mut inner = HashMap::<usize, Vec<ReservedWord>>::new();

        for reserved_word in RESERVED_WORDS.iter() {
            let len = reserved_word.name.len();
            if !inner.contains_key(&len) {
                inner.insert(len, vec![]);
            }
            inner.get_mut(&len).unwrap().push(reserved_word.clone());
        }

        Self { inner }
    }

    fn to_vec_of_vecs(&self) -> Vec<Vec<ReservedWord>> {
        let max_len = *self.inner.keys().max().unwrap();
        (0..=max_len)
            .map(|len| {
                let words = self.inner.get(&len);
                if let Some(words) = words {
                    let mut words = words.clone();
                    words.sort_by(|x, y| x.name.cmp(y.name));
                    return words;
                }
                vec![]
            })
            .collect::<Vec<_>>()
    }

    pub(crate) fn write(&self) {
        std::fs::write("src/reserved_words/list.rs", self.contents()).unwrap();
    }

    fn contents(&self) -> String {
        format!(
            "use crate::lex_states::*;
use crate::reserved_words::ReservedWord;
use crate::Lexer;

pub(crate) const RESERVED_WORDS: &[&[ReservedWord]] = {:#?};",
            self
        )
    }
}

impl std::fmt::Debug for ReservedWordsList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "&[
    {}
]",
            self.to_vec_of_vecs()
                .iter()
                .map(|x| format!("&{:#?}", x))
                .collect::<Vec<_>>()
                .join(",\n    ")
        )
    }
}

impl std::fmt::Debug for ReservedWord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ReservedWord")
            .field("name", &self.name)
            .field("id", &self.id)
            .field("modifier_id", &self.modifier_id)
            .field("state", &self.state)
            .finish()
    }
}

impl std::fmt::Debug for RawString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.s)
    }
}
