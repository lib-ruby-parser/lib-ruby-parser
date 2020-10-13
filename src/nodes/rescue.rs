use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Rescue {
    pub body: Option<Box<Node>>,
    pub rescue_bodies: Vec<Node>,
    pub else_: Option<Box<Node>>,

    pub expression_l: Range,
    pub else_l: Range,
}
