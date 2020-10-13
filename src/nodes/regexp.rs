use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Regexp {
    pub parts: Vec<Node>,
    pub options: Box<Node>,

    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}
