use crate::source::Range;
use crate::Node;

// TODO: remove
#[derive(Debug, Clone, PartialEq)]
pub struct MatchWithTrailingComma {
    pub match_: Box<Node>,

    pub expression_l: Range,
}
