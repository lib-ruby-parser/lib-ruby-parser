use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Optarg {
    pub name: String,
    pub default: Box<Node>,
    pub name_l: Range,
    pub operator_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Optarg {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_str(&self.name);
        result.push_node(&self.default);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "optarg"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.operator_l.print("operator");
        self.name_l.print("name");
        self.default.inner().print_with_locs();
    }
}
