#[derive(Debug, Clone, Default)]
pub struct LexState {
    value: usize
}

impl LexState {
    pub fn new() -> Self {
        Self { value: lex_states::EXPR_BEG }
    }

    pub fn is_some(&self, states: usize) -> bool {
        (self.value & states) != 0
    }

    pub fn is_all(&self, states: usize) -> bool {
        (self.value & states) == states
    }

    pub fn set(&mut self, value: usize) {
        self.value = value
    }
}

pub mod lex_states {
    pub const EXPR_BEG: usize = 1 << 0;
    pub const EXPR_END: usize = 1 << 1;
    pub const EXPR_ENDARG: usize = 1 << 2;
    pub const EXPR_ENDFN: usize = 1 << 3;
    pub const EXPR_ARG: usize = 1 << 4;
    pub const EXPR_CMDARG: usize = 1 << 5;
    pub const EXPR_MID: usize = 1 << 6;
    pub const EXPR_FNAME: usize = 1 << 7;
    pub const EXPR_DOT: usize = 1 << 8;
    pub const EXPR_CLASS: usize = 1 << 9;
    pub const EXPR_LABEL: usize = 1 << 10;
    pub const EXPR_LABELED: usize = 1 << 11;
    pub const EXPR_FITEM: usize = 1 << 12;
    pub const EXPR_VALUE: usize = EXPR_BEG;
    pub const EXPR_BEG_ANY: usize = EXPR_BEG | EXPR_MID | EXPR_CLASS;
    pub const EXPR_ARG_ANY: usize = EXPR_ARG | EXPR_CMDARG;
    pub const EXPR_END_ANY: usize = EXPR_END | EXPR_ENDARG | EXPR_ENDFN;
    pub const EXPR_NONE: usize = 0;
}
