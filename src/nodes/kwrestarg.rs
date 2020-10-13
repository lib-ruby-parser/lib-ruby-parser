use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwrestarg {
    pub name: Option<String>,

    pub name_l: Option<Range>,
    pub expression_l: Range,
}
