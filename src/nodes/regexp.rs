use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Regexp {
    pub parts: Vec<Node>,
    pub options: Box<Node>,
    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Regexp {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_nodes(&self.parts);
        result.push_node(&self.options);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "regexp"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        self.begin_l.print("begin");
        self.options.inner().print_with_locs();
        for node in self.parts.iter() {
            node.inner().print_with_locs();
        }
    }
}
