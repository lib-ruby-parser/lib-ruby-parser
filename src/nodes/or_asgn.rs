use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct OrAsgn {
    pub recv: Box<Node>,
    pub value: Box<Node>,
    pub expression_l: Range,
    pub operator_l: Range,
}

impl InnerNode for OrAsgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.recv);
        result.push_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "or_asgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.operator_l.print("operator");
        self.expression_l.print("expression");
        self.value.inner().print_with_locs();
        self.recv.inner().print_with_locs();
    }
}
