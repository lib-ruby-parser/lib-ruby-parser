use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Lvar {
    pub name: String,

    pub expression_l: Range,
}
