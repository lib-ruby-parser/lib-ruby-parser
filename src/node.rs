use crate::source::Range;
use crate::source::map::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Begin { statements: Vec<Node>, loc: CollectionMap },
    Int { value: String, loc: OperatorMap },
    Send { receiver: Option<Box<Node>>, operator: String, args: Vec<Node>, loc: SendMap },
    Nil { loc: Map },
    True { loc: Map },
    False { loc: Map },
    Self_ { loc: Map },
    __FILE__ { loc: Map },
    __LINE__ { loc: Map },
    __ENCODING__ { loc: Map },
    Preexe { body: Box<Node>, loc: KeywordMap },
    Lvar { name: String, loc: VariableMap },
    Rescue { body: Option<Box<Node>>, rescue_bodies: Vec<Node>, else_: Option<Box<Node>>, loc: ConditionMap },
    Ensure { body: Option<Box<Node>>, ensure: Box<Node>, loc: ConditionMap },
    KwBegin { statements: Vec<Node>, loc: CollectionMap },

}

impl Node {
    pub fn expression(&self) -> &Range {
        match self {
            Self::Begin { loc, .. } => &loc.expression,
            Self::Int { loc, .. } => &loc.expression,
            Self::Send { loc, .. } => &loc.expression,
            Self::Nil { loc, .. } => &loc.expression,
            Self::True { loc, .. } => &loc.expression,
            Self::False { loc, .. } => &loc.expression,
            Self::Self_ { loc, .. } => &loc.expression,
            Self::__FILE__ { loc, .. } => &loc.expression,
            Self::__LINE__ { loc, .. } => &loc.expression,
            Self::__ENCODING__ { loc, .. } => &loc.expression,
            Self::Preexe { loc, .. } => &loc.expression,
            Self::Lvar { loc, .. } => &loc.expression,
            Self::Rescue { loc, .. } => &loc.expression,
            Self::Ensure { loc, .. } => &loc.expression,
            Self::KwBegin { loc, .. } => &loc.expression,
        }
    }
}
