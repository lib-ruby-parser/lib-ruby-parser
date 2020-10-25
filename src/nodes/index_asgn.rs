use crate::nodes::InnerNode;
use crate::nodes::InspectVec;
use crate::source::Range;
use crate::Node;

#[derive(Debug, Clone, PartialEq)]
pub struct IndexAsgn {
    pub recv: Box<Node>,
    pub indexes: Vec<Node>,
    pub value: Option<Box<Node>>,
    pub begin_l: Range,
    pub end_l: Range,
    pub operator_l: Option<Range>,
    pub expression_l: Range,
}

impl InnerNode for IndexAsgn {
    fn expression(&self) -> &Range {
        &self.expression_l
    }


    fn inspected_children(&self, indent: usize) -> Vec<String> {
        let mut result = InspectVec::new(indent);
        result.push_node(&self.recv);
        result.push_nodes(&self.indexes);
        result.push_maybe_node(&self.value);
        result.strings()
    }

    fn str_type(&self) -> &'static str {
        "indexasgn"
    }

    fn print_with_locs(&self) {
        println!("{}", self.inspect(0));
        self.expression_l.print("expression");
        if let Some(range) = &self.operator_l {
            range.print("operator");
        }
        self.end_l.print("end");
        self.begin_l.print("begin");
        if let Some(node) = &self.value {
            node.inner().print_with_locs();
        }
        for node in self.indexes.iter() {
            node.inner().print_with_locs();
        }
        self.recv.inner().print_with_locs();
    }
}
