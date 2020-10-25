use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct UnlessGuard {
    pub cond: Box<Node>,
    pub keyword_l: Range,
    pub expression_l: Range,
}

impl InnerNode for UnlessGuard {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "unless_guard"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.keyword_l.print("keyword");
        self.cond.inner().print_with_locs();
    }
}
