use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Next {
    pub args: Vec<Node>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
