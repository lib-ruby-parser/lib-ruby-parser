use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexAsgn {
    pub recv: Box<Node>,
    pub indexes: Vec<Node>,
    pub value: Box<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
