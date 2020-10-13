use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Gvar {
    pub name: String,

    pub expression_l: Range,
}
