use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct CaseMatch {
    pub expr: Box<Node>,
    pub in_bodies: Vec<Node>,
    pub else_body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub else_l: Option<Range>,
    pub end_l: Range,
    pub expression_l: Range,
}

impl InnerNode for CaseMatch {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.expr);
        result.push_nodes(&self.in_bodies);
        result.push_maybe_node_or_nil(&self.else_body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "case_match"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.end_l.print("end");
        if let Some(range) = &self.else_l {
            range.print("else");
        }
        self.keyword_l.print("keyword");
        if let Some(node) = &self.else_body {
            node.inner().print_with_locs();
        }
        for node in self.in_bodies.iter() {
            node.inner().print_with_locs();
        }
        self.expr.inner().print_with_locs();
    }
}
