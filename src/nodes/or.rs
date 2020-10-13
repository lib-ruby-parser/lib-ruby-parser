use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Or {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}
