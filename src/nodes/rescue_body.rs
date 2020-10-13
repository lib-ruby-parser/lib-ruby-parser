use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct RescueBody {
    pub exc_list: Option<Box<Node>>,
    pub exc_var: Option<Box<Node>>,
    pub body: Option<Box<Node>>,

    pub expression_l: Range,
    pub keyword_l: Range,
    pub assoc_l: Option<Range>,
    pub begin_l: Option<Range>,
}
