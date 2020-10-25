use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct RescueBody {
    pub exc_list: Option<Box<Node>>,
    pub exc_var: Option<Box<Node>>,
    pub body: Option<Box<Node>>,
    pub keyword_l: Range,
    pub assoc_l: Option<Range>,
    pub begin_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for RescueBody {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.exc_list);
        result.push_maybe_node_or_nil(&self.exc_var);
        result.push_maybe_node_or_nil(&self.body);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "resbody"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.begin_l {
            range.print("begin");
        }
        if let Some(range) = &self.assoc_l {
            range.print("assoc");
        }
        self.keyword_l.print("keyword");
        if let Some(node) = &self.body {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.exc_var {
            node.inner().print_with_locs();
        }
        if let Some(node) = &self.exc_list {
            node.inner().print_with_locs();
        }
    }
}
