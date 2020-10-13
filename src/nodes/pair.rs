use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Pair {
    pub key: Box<Node>,
    pub value: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}
