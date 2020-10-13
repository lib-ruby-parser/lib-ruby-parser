use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Int {
    pub value: String,
    pub expression_l: Range,
}
