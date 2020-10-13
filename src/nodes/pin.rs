use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Pin {
    pub var: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}
