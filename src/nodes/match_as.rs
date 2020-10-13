use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchAs {
    pub value: Box<Node>,
    pub as_: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}
