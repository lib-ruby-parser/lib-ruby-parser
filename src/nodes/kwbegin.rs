use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct KwBegin {
    pub statements: Vec<Node>,

    pub expression_l: Range,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
}
