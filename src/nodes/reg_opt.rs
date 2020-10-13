use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct RegOpt {
    pub options: Vec<char>,

    pub expression_l: Range,
}
