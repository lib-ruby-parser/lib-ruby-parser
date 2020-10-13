use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub to: Box<Node>,
    pub from: Box<Node>,

    pub keyword_l: Range,
    pub expression_l: Range,
}
