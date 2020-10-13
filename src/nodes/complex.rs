use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Complex {
    pub value: String,

    pub expression_l: Range,
}
