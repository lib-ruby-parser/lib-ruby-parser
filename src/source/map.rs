use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Map {
    pub expression: Range,
}

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
pub struct VariableMap {
    pub expression: Range,
    pub name: Range,
    pub operator: Option<Range>,
}
