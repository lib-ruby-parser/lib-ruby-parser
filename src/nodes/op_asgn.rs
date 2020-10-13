use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct OpAsgn {
    pub recv: Box<Node>,
    pub value: Box<Node>,

    pub expression_l: Range,
    pub operator_l: Range,
}
