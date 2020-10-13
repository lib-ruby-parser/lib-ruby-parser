use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct FindPattern {
    pub elements: Vec<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
