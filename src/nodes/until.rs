use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Until {
    pub cond: Box<Node>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub begin_l: Option<Range>,
    pub end_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for Until {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.cond);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "until"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.end_l {
            range.print("end");
        }
        if let Some(range) = &self.begin_l {
            range.print("begin");
        }
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        self.cond.inner().print_with_locs();
    }
}
