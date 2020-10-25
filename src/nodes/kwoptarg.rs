use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Kwoptarg {
    pub name: String,
    pub default: Box<Node>,
    pub name_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Kwoptarg {
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
        "kwoptarg"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.name_l.print("name");
        self.default.inner().print_with_locs();
    }
}
