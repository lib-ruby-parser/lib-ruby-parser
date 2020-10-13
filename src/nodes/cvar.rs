use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Cvar {
    pub name: String,

    pub expression_l: Range,
}
