#[derive(Clone)]
pub struct LexState {
    value: i32
}

impl LexState {
    pub fn new() -> Self {
        Self { value: lex_states::EXPR_BEG }
    }

    pub fn is(&self, value: i32) -> bool {
        self.value == value
    }

    pub fn is_some(&self, states: i32) -> bool {
        (self.value & states) != 0
    }

    pub fn is_all(&self, states: i32) -> bool {
        (self.value & states) == states
    }

    pub fn set(&mut self, value: i32) {
        self.value = value
    }

    pub fn get(&self) -> i32 {
        self.value
    }
}

impl Default for LexState {
    fn default() -> Self {
        Self { value: lex_states::EXPR_BEG }
    }
}

pub mod lex_states {
    pub const EXPR_BEG: i32 = 1 << 0;
    pub const EXPR_END: i32 = 1 << 1;
    pub const EXPR_ENDARG: i32 = 1 << 2;
    pub const EXPR_ENDFN: i32 = 1 << 3;
    pub const EXPR_ARG: i32 = 1 << 4;
    pub const EXPR_CMDARG: i32 = 1 << 5;
    pub const EXPR_MID: i32 = 1 << 6;
    pub const EXPR_FNAME: i32 = 1 << 7;
    pub const EXPR_DOT: i32 = 1 << 8;
    pub const EXPR_CLASS: i32 = 1 << 9;
    pub const EXPR_LABEL: i32 = 1 << 10;
    pub const EXPR_LABELED: i32 = 1 << 11;
    pub const EXPR_FITEM: i32 = 1 << 12;
    pub const EXPR_VALUE: i32 = EXPR_BEG;
    pub const EXPR_BEG_ANY: i32 = EXPR_BEG | EXPR_MID | EXPR_CLASS;
    pub const EXPR_ARG_ANY: i32 = EXPR_ARG | EXPR_CMDARG;
    pub const EXPR_END_ANY: i32 = EXPR_END | EXPR_ENDARG | EXPR_ENDFN;
    pub const EXPR_NONE: i32 = 0;
}

impl std::fmt::Debug for LexState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut states: Vec<&'static str> = vec![];

        if self.is_some(lex_states::EXPR_BEG) { states.push("EXPR_BEG") }
        if self.is_some(lex_states::EXPR_END) { states.push("EXPR_END") }
        if self.is_some(lex_states::EXPR_ENDARG) { states.push("EXPR_ENDARG") }
        if self.is_some(lex_states::EXPR_ENDFN) { states.push("EXPR_ENDFN") }
        if self.is_some(lex_states::EXPR_ARG) { states.push("EXPR_ARG") }
        if self.is_some(lex_states::EXPR_CMDARG) { states.push("EXPR_CMDARG") }
        if self.is_some(lex_states::EXPR_MID) { states.push("EXPR_MID") }
        if self.is_some(lex_states::EXPR_FNAME) { states.push("EXPR_FNAME") }
        if self.is_some(lex_states::EXPR_DOT) { states.push("EXPR_DOT") }
        if self.is_some(lex_states::EXPR_CLASS) { states.push("EXPR_CLASS") }
        if self.is_some(lex_states::EXPR_LABEL) { states.push("EXPR_LABEL") }
        if self.is_some(lex_states::EXPR_FITEM) { states.push("EXPR_FITEM") }
        if self.is_some(lex_states::EXPR_NONE) { states.push("EXPR_NONE") }

        if self.is_some(lex_states::EXPR_VALUE) { states.push("++ EXPR_VALUE ++") }
        if self.is_some(lex_states::EXPR_BEG_ANY) { states.push("++ EXPR_BEG_ANY ++") }
        if self.is_some(lex_states::EXPR_END_ANY) { states.push("++ EXPR_END_ANY ++") }
        if self.is_some(lex_states::EXPR_END_ANY) { states.push("++ EXPR_END_ANY ++") }

        f.write_str(&states.join("|"))
    }
}
