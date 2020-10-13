use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct NthRef {
    pub name: String,

    pub expression_l: Range,
}
