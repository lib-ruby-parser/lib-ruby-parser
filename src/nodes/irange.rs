use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Irange {
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,

    pub operator_l: Range,
    pub expression_l: Range,
}
