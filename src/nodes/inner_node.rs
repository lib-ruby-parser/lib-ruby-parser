use crate::containers::{maybe_ptr::AsOption, List, MaybePtr};
use crate::Loc;
use crate::Node;
use crate::StringValue;

pub trait InnerNode {
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
        if let Some(string) = string {
            self.strings.push(format!(", {:?}", string));
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

    pub(crate) fn push_maybe_node(&mut self, node: &MaybePtr<Node>) {
        if let Some(node) = node.as_option() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_regex_options(&mut self, node: &MaybePtr<Node>) {
        if let Some(node) = node.as_option() {
            self.push_node(node)
        } else {
            self.strings.push(format!(
                ",\n{}{}",
                "  ".repeat(self.indent + 1),
                "s(:regopt)"
            ))
        }
    }

    pub(crate) fn push_maybe_node_or_nil(&mut self, node: &MaybePtr<Node>) {
        if let Some(node) = node.as_option() {
            self.push_node(node)
        } else {
            self.push_nil()
        }
    }

    pub(crate) fn push_nodes(&mut self, nodes: &List<Node>) {
        for node in nodes.iter() {
            self.push_node(node)
        }
    }

    pub(crate) fn push_chars(&mut self, chars: &[char]) {
        for c in chars {
            self.push_str(&format!("{}", c));
        }
    }

    pub(crate) fn push_string_value(&mut self, s: &StringValue) {
        self.push_str(&s.bytes.to_string_lossy())
    }

    pub(crate) fn strings(&mut self) -> Vec<String> {
        std::mem::take(&mut self.strings)
    }
}
