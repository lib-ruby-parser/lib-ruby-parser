use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Redo {
    pub expression_l: Range,
}
