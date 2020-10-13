use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstPattern {
    pub const_: Box<Node>,
    pub pattern: Box<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
