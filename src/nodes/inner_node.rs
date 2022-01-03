use crate::Bytes;
use crate::Loc;
use crate::Node;

pub trait InnerNode: std::fmt::Debug {
    fn expression(&self) -> &Loc;
    fn str_type(&self) -> &'static str;
    fn inspected_children(&self, indent: usize) -> Vec<String>;

    fn inspect(&self, indent: usize) -> String {
        let indented = "  ".repeat(indent);
        let mut sexp = format!("{}s(:{}", indented, self.str_type());

        for child in self.inspected_children(indent) {
            sexp.push_str(&child);
        }

        sexp.push(')');

        sexp
    }

    fn print_with_locs(&self);
}

pub(crate) struct InspectVec {
    indent: usize,
    strings: Vec<String>,
}

impl InspectVec {
    pub(crate) fn new(indent: usize) -> Self {
        Self {
            indent,
            strings: vec![],
        }
    }

    pub(crate) fn push_str(&mut self, string: &str) {
        self.strings.push(format!(", {:?}", string));
    }

    pub(crate) fn push_raw_str(&mut self, string: &str) {
        self.strings.push(format!(", {}", string));
    }

    pub(crate) fn push_maybe_str(&mut self, string: &Option<String>) {
        if let Some(string) = string.as_ref() {
            self.strings.push(format!(", {:?}", string));
        }
    }

    pub(crate) fn push_maybe_str_or_nil(&mut self, string: &Option<String>) {
        if let Some(string) = string.as_ref() {
            self.push_str(string)
        } else {
            self.push_nil()
        }
    }

    pub(crate) fn push_nil(&mut self) {
        self.strings.push(", nil".to_string());
    }

    pub(crate) fn push_u8(&mut self, n: &u8) {
        self.strings.push(format!(", {}", n))
    }

    pub(crate) fn push_node(&mut self, node: &Node) {
        self.strings
            .push(format!(",\n{}", node.inspect(self.indent + 1)))
    }

    pub(crate) fn push_maybe_node(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_regex_options(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        } else {
            self.strings.push(format!(
                ",\n{}{}",
                "  ".repeat(self.indent + 1),
                "s(:regopt)"
            ))
        }
    }

    pub(crate) fn push_maybe_node_or_nil(&mut self, node: &Option<Box<Node>>) {
        if let Some(node) = node.as_ref() {
            self.push_node(node)
        } else {
            self.push_nil()
        }
    }

    pub(crate) fn push_nodes(&mut self, nodes: &[Node]) {
        for node in nodes.iter() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_chars(&mut self, chars: &Option<String>) {
        if let Some(chars) = chars.as_ref() {
            for c in chars.as_str().chars() {
                self.push_str(&format!("{}", c));
            }
        }
    }

    pub(crate) fn push_string_value(&mut self, bytes: &Bytes) {
        self.push_str(&bytes.to_string_lossy())
    }

    pub(crate) fn strings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.strings)
    }
}
