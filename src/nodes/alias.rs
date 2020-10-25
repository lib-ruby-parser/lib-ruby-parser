use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Alias {
    pub to: Box<Node>,
    pub from: Box<Node>,
    pub keyword_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Alias {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.to);
        result.push_node(&self.from);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "alias"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.keyword_l.print("keyword");
        self.from.inner().print_with_locs();
        self.to.inner().print_with_locs();
    }
}
