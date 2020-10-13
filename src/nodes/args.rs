use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub args: Vec<Node>,

    pub expression_l: Range,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
}
