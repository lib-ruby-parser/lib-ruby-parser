use lib_ruby_parser_nodes::{Node, NodeField, NodeFieldType};

pub(crate) struct Visitor<'a> {
    nodes: &'a [Node],
}

impl<'a> Visitor<'a> {
    pub(crate) fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    pub(crate) fn write(&self) {
        std::fs::write("src/traverse/visitor/visit_gen.rs", self.contents()).unwrap()
    }

    fn contents(&self) -> String {
        format!(
            "use crate::nodes::*;
use crate::traverse::visitor::{{Item, Visit, Visitor}};
use crate::Node;

/// Trait that must be implement to observe actions
/// that are performed by `Visitor` while it traverses given `Node`.
pub trait Observer {{
    {observer_methods}

    /// Caled when entering any `Node`
    #[allow(unused_variables)]
    fn on_node(&mut self, node: &Node) {{}}

    /// Called when exiting any `Node`
    #[allow(unused_variables)]
    fn on_node_moving_up(&mut self, node: &Node) {{}}

    /// Called when entering any optional `Node`
    #[allow(unused_variables)]
    fn on_option_node(&mut self, node: &Option<Box<Node>>) {{}}

    /// Called when entering any `Vec<Node>`
    #[allow(unused_variables)]
    fn on_node_list(&mut self, nodes: &[Node]) {{}}

    /// Called when entering any AST node,
    /// `subitem` is different for different `Node` fields,
    /// check documentation of `traverse::visitor::Item`
    #[allow(unused_variables)]
    fn on_subitem(&mut self, subitem: Item) {{}}

    /// Called when exiting any AST node,
    /// `subitem` is different for different `Node` fields,
    /// check documentation of `traverse::visitor::Item`
    #[allow(unused_variables)]
    fn on_subitem_moving_up(&mut self, subitem: Item) {{}}
}}

impl<TObserver: Observer> Visit<&Node> for Visitor<TObserver> {{
    fn visit(&mut self, node: &Node, visit_as: Item) {{
        self.observer.on_subitem(visit_as);
        self.observer.on_node(node);

        {visit_branches}

        self.observer.on_node_moving_up(&node);
        self.observer.on_subitem_moving_up(visit_as);
    }}
}}

impl<T> Visitor<T>
where
    T: Observer,
{{
    {visitor_methods}
}}
",
            observer_methods = self.observer_methods().join("\n\n    "),
            visit_branches = self.visit_branches().join("\n        "),
            visitor_methods = self.visitor_methods().join("\n\n    ")
        )
    }

    fn observer_methods(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "/// Invoked by a `Visitor` on entering into `{struct_name}` node.
    #[allow(unused_variables)]
    fn on_{lower}(&mut self, node: &{struct_name}) {{}}",
                    lower = node.lower_name(),
                    struct_name = node.struct_name
                )
            })
            .collect()
    }

    fn visit_branches(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "if let Some(inner) = node.as_{lower}() {{
            self.visit_{lower}(inner)
        }}",
                    lower = node.lower_name()
                )
            })
            .collect()
    }
    fn visitor_methods(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "fn visit_{lower}(&mut self, node: &{struct_name}) {{
        self.observer.on_{lower}(node);

        {visit_children}
    }}",
                    struct_name = node.struct_name,
                    lower = node.lower_name(),
                    visit_children = NodeWrapper::new(node).visit_children().join("\n        ")
                )
            })
            .collect()
    }
}

struct NodeWrapper<'a> {
    node: &'a Node,
}

impl<'a> NodeWrapper<'a> {
    pub(crate) fn new(node: &'a Node) -> Self {
        Self { node }
    }

    pub(crate) fn visit_children(&self) -> Vec<String> {
        self.node
            .fields
            .0
            .iter()
            .filter_map(|f| {
                match f.field_type {
                    NodeFieldType::Node => {}
                    NodeFieldType::Nodes => {}
                    NodeFieldType::MaybeNode => {}
                    NodeFieldType::RegexOptions => {}

                    NodeFieldType::Loc
                    | NodeFieldType::MaybeLoc
                    | NodeFieldType::Str
                    | NodeFieldType::MaybeStr
                    | NodeFieldType::Chars
                    | NodeFieldType::StringValue
                    | NodeFieldType::U8
                    | NodeFieldType::Usize
                    | NodeFieldType::RawString => return None,
                }

                let variant = field_name_to_variant(self.node, f);

                Some(format!(
                    "self.visit(node.get_{field_name}(), Item::{variant});",
                    field_name = f.field_name,
                    variant = variant
                ))
            })
            .collect()
    }
}

fn field_name_to_variant(node: &Node, field: &NodeField) -> String {
    match (&node.str_type[..], &field.field_name[..]) {
        (_, "statements") => return "Stmts".to_string(),
        (_, "call") => return "MethodCall".to_string(),
        (_, "default") => return "DefaultValue".to_string(),
        (_, "items") => return "MlhsItems".to_string(),
        ("when", "patterns") => return "Args".to_string(),
        ("undef", "names") => return "Args".to_string(),
        ("args", "args") => return "Arglist".to_string(),
        ("procarg0", "args") => return "Arglist".to_string(),
        ("rescue", "else_") => return "ElseBody".to_string(),
        _ => {}
    };
    capitalize_field_name(&field.field_name)
}

fn capitalize_field_name(s: &str) -> String {
    s.split("_").map(|word| capitalize_word(word)).collect()
}

fn capitalize_word(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub(crate) fn codegen() {
    let nodes = lib_ruby_parser_nodes::nodes().0;

    Visitor::new(&nodes).write();
}
