use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectionMap {
    pub expression: Range,
    pub begin: Option<Range>,
    pub end: Option<Range>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct OperatorMap {
    pub expression: Range,
    pub operator: Option<Range>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct SendMap {
    pub expression: Range,
    pub dot: Option<Range>,
    pub selector: Option<Range>,
    pub operator: Option<Range>,
    pub begin: Option<Range>,
    pub end: Option<Range>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    None,
    Begin { statements: Vec<Node>, loc: CollectionMap },
    Int { value: String, loc: OperatorMap },
    Send { receiver: Option<Box<Node>>, operator: String, args: Vec<Node>, loc: SendMap },
}

impl Node {
    pub fn expression(&self) -> &Range {
        match self {
            Self::None => panic!("None node doesn't have any locs"),
            Self::Begin { loc, .. } => &loc.expression,
            Self::Int { loc, .. } => &loc.expression,
            Self::Send { loc, .. } => &loc.expression,
        }
    }
}
