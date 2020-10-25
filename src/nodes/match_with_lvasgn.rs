use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct MatchWithLvasgn {
    pub re: Box<Node>,
    pub value: Box<Node>,
    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for MatchWithLvasgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.re);
        result.push_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "match_with_lvasgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.operator_l.print("operator");
        self.value.inner().print_with_locs();
        self.re.inner().print_with_locs();
    }
}
