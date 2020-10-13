use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Rational {
    pub value: String,
    pub expression_l: Range,
}
