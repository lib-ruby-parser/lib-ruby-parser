use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayPatternWithTail {
    pub elements: Vec<Node>,

    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
    pub expression_l: Range,
}
