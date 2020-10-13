use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Case {
    pub expr: Option<Box<Node>>,
    pub when_bodies: Vec<Node>,
    pub else_body: Option<Box<Node>>,

    pub keyword_l: Range,
    pub else_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}
