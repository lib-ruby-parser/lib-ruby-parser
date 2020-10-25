use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct ConstPattern {
    pub const_: Box<Node>,
    pub pattern: Box<Node>,
    pub begin_l: Range,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for ConstPattern {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.const_);
        result.push_node(&self.pattern);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "const_pattern"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        self.begin_l.print("begin");
        self.pattern.inner().print_with_locs();
        self.const_.inner().print_with_locs();
    }
}
