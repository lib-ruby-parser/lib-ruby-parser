use crate::nodes::InnerNode;
use crate::source::Range;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchNilPattern {
    pub operator_l: Range,
    pub name_l: Range,
    pub expression_l: Range,
}

impl InnerNode for MatchNilPattern {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, _indent: usize) -> Vec<String> {
        vec![]
    }

    fn str_type(&self) -> &'static str {
        "match_nil_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.name_l.print("name");
        self.operator_l.print("operator");
    }
}
