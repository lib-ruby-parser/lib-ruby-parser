use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Or {
    pub lhs: Box<Node>,
    pub rhs: Box<Node>,
    pub expression_l: Range,
    pub operator_l: Range,
}

impl InnerNode for Or {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.lhs);
        result.push_node(&self.rhs);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "or"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.operator_l.print("operator");
        self.expression_l.print("expression");
        self.rhs.inner().print_with_locs();
        self.lhs.inner().print_with_locs();
    }
}
