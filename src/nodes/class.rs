use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
    pub name: Box<Node>,
    pub superclass: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub operator_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}
