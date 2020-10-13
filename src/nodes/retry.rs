use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Retry {
    pub expression_l: Range,
}
