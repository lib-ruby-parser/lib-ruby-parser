use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct EmptyElse {
    pub expression_l: Range,
}
