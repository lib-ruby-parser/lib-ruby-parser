use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchCurrentLine {
    pub re: Box<Node>,

    pub expression_l: Range,
}
