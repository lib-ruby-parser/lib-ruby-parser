use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Float {
    pub value: String,

    pub expression_l: Range,
}
