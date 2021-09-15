use crate::maybe_byte::MaybeByte;

/// State of the lexer
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct LexState {
    pub(crate) value: i32,
}

impl LexState {
    pub(crate) fn is(&self, value: i32) -> bool {
        self.value == value
    }

    pub(crate) fn is_some(&self, states: i32) -> bool {
        (self.value & states) != 0
    }

    pub(crate) fn is_all(&self, states: i32) -> bool {
        (self.value & states) == states
    }

    /// Sets state to given `value`
    pub fn set(&mut self, value: i32) {
        self.value = value
    }

    pub(crate) fn get(&self) -> i32 {
        self.value
    }

    pub(crate) fn is_after_operator(&self) -> bool {
        self.is_some(EXPR_FNAME | EXPR_DOT)
    }
    pub(crate) fn is_end(&self) -> bool {
        self.is_some(EXPR_END_ANY)
    }
    pub(crate) fn is_arg(&self) -> bool {
        self.is_some(EXPR_ARG_ANY)
    }
    pub(crate) fn is_label_possible(&self, cmd_state: bool) -> bool {
        (self.is_some(EXPR_LABEL | EXPR_ENDFN) && !cmd_state) || self.is_arg()
    }
    pub(crate) fn is_spacearg(&self, c: MaybeByte, space_seen: bool) -> bool {
        self.is_arg() && space_seen && !c.is_space()
    }
    pub(crate) fn is_beg(&self) -> bool {
        self.is_some(EXPR_BEG_ANY) || self.is_all(EXPR_ARG | EXPR_LABELED)
    }
}

impl Default for LexState {
    fn default() -> Self {
        Self { value: EXPR_BEG }
    }
}

/// Mod with all known lex states
pub mod lex_states {
    /// EXPR_BEG state in MRI
    pub const EXPR_BEG: i32 = 1 << 0;

    /// EXPR_END state in MRI
    pub const EXPR_END: i32 = 1 << 1;

    /// EXPR_ENDARG state in MRI
    pub const EXPR_ENDARG: i32 = 1 << 2;

    /// EXPR_ENDFN state in MRI
    pub const EXPR_ENDFN: i32 = 1 << 3;

    /// EXPR_ARG state in MRI
    pub const EXPR_ARG: i32 = 1 << 4;

    /// EXPR_CMDARG state in MRI
    pub const EXPR_CMDARG: i32 = 1 << 5;

    /// EXPR_MID state in MRI
    pub const EXPR_MID: i32 = 1 << 6;

    /// EXPR_FNAME state in MRI
    pub const EXPR_FNAME: i32 = 1 << 7;

    /// EXPR_DOT state in MRI
    pub const EXPR_DOT: i32 = 1 << 8;

    /// EXPR_CLASS state in MRI
    pub const EXPR_CLASS: i32 = 1 << 9;

    /// EXPR_LABEL state in MRI
    pub const EXPR_LABEL: i32 = 1 << 10;

    /// EXPR_LABELED state in MRI
    pub const EXPR_LABELED: i32 = 1 << 11;

    /// EXPR_FITEM state in MRI
    pub const EXPR_FITEM: i32 = 1 << 12;

    /// EXPR_VALUE state in MRI
    pub const EXPR_VALUE: i32 = EXPR_BEG;

    /// EXPR_BEG_ANY state in MRI
    pub const EXPR_BEG_ANY: i32 = EXPR_BEG | EXPR_MID | EXPR_CLASS;

    /// EXPR_ARG_ANY state in MRI
    pub const EXPR_ARG_ANY: i32 = EXPR_ARG | EXPR_CMDARG;

    /// EXPR_END_ANY state in MRI
    pub const EXPR_END_ANY: i32 = EXPR_END | EXPR_ENDARG | EXPR_ENDFN;

    /// EXPR_NONE state in MRI
    pub const EXPR_NONE: i32 = 0;
}
use lex_states::*;

impl std::fmt::Debug for LexState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut states: Vec<&'static str> = vec![];

        if self.is_some(EXPR_BEG) {
            states.push("EXPR_BEG")
        }
        if self.is_some(EXPR_END) {
            states.push("EXPR_END")
        }
        if self.is_some(EXPR_ENDARG) {
            states.push("EXPR_ENDARG")
        }
        if self.is_some(EXPR_ENDFN) {
            states.push("EXPR_ENDFN")
        }
        if self.is_some(EXPR_ARG) {
            states.push("EXPR_ARG")
        }
        if self.is_some(EXPR_CMDARG) {
            states.push("EXPR_CMDARG")
        }
        if self.is_some(EXPR_MID) {
            states.push("EXPR_MID")
        }
        if self.is_some(EXPR_FNAME) {
            states.push("EXPR_FNAME")
        }
        if self.is_some(EXPR_DOT) {
            states.push("EXPR_DOT")
        }
        if self.is_some(EXPR_CLASS) {
            states.push("EXPR_CLASS")
        }
        if self.is_some(EXPR_LABEL) {
            states.push("EXPR_LABEL")
        }
        if self.is_some(EXPR_FITEM) {
            states.push("EXPR_FITEM")
        }
        if self.is_some(EXPR_NONE) {
            states.push("EXPR_NONE")
        }

        if self.is_some(EXPR_VALUE) {
            states.push("Also(EXPR_VALUE)")
        }
        if self.is_some(EXPR_BEG_ANY) {
            states.push("Also(EXPR_BEG_ANY)")
        }
        if self.is_some(EXPR_END_ANY) {
            states.push("Also(EXPR_END_ANY)")
        }
        if self.is_some(EXPR_END_ANY) {
            states.push("Also(EXPR_END_ANY)")
        }

        f.write_str(&states.join("|"))
    }
}
