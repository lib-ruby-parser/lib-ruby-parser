use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwsplat {
    pub value: Option<Box<Node>>,

    pub operator_l: Range,
    pub expression_l: Range,
}
