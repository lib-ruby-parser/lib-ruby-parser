use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchWithLvasgn {
    pub re: Box<Node>,
    pub arg: Box<Node>,

    pub operator_l: Range,
    pub expression_l: Range,
}
