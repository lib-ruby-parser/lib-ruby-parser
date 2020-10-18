use crate::source::Range;
use crate::Node;

pub trait InnerNode {
    fn expression(&self) -> &Range;
    fn str_type(&self) -> &'static str;
    fn inspected_children(&self, indent: usize) -> Vec<String>;

    fn inspect(&self, indent: usize) -> String {
        let indented = "  ".repeat(indent);
        let mut sexp = format!("{}s(:{}", indented, self.str_type());

        for child in self.inspected_children(indent) {
            sexp.push_str(&child);
        }

        sexp.push_str(")");

        sexp
    }
}

pub struct InspectVec {
    indent: usize,
    strings: Vec<String>,
}

impl InspectVec {
    pub fn new(indent: usize) -> Self {
        Self {
            indent,
            strings: vec![],
        }
    }

    pub fn push_str(&mut self, string: &str) {
        self.strings.push(format!(", {:?}", string));
    }

    pub fn push_nil(&mut self) {
        self.strings.push(", nil".to_owned());
    }

    pub fn push_u8(&mut self, n: u8) {
        self.strings.push(format!(", {}", n))
    }

    pub fn push_usize(&mut self, n: usize) {
        self.strings.push(format!(", {}", n))
    }

    pub fn push_node(&mut self, node: &Node) {
        self.strings
            .push(format!(",\n{}", node.inspect(self.indent + 1)))
    }

    pub fn push_maybe_node(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node {
            self.push_node(node)
        }
    }

    pub fn push_maybe_node_or_nil(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node {
            self.push_node(node)
        } else {
            self.push_nil()
        }
    }

    pub fn push_nodes(&mut self, nodes: &Vec<Node>) {
        for node in nodes {
            self.push_node(node)
        }
    }

    pub fn strings(self) -> Vec<String> {
        self.strings
    }
}
