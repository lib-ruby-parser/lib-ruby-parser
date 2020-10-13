use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Heredoc {
    pub parts: Vec<Node>,

    pub heredoc_body_l: Range,
    pub heredoc_end_l: Range,
    pub expression_l: Range,
}
