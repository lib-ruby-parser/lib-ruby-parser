use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct Sym {
    pub name: String,

    pub expression_l: Range,
    pub begin_l: Range,
}
