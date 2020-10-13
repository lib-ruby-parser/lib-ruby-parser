use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Ivar {
    pub name: String,

    pub expression_l: Range,
}
