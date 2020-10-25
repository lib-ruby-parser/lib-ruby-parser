use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct Const {
    pub scope: Option<Box<Node>>,
    pub name: String,
    pub double_colon_l: Option<Range>,
    pub name_l: Range,
    pub expression_l: Range,
}

impl InnerNode for Const {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_maybe_node_or_nil(&self.scope);
        result.push_str(&self.name);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "const"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        self.name_l.print("name");
        if let Some(range) = &self.double_colon_l {
            range.print("double_colon");
        }
        if let Some(node) = &self.scope {
            node.inner().print_with_locs();
        }
    }
}
