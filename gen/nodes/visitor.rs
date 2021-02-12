use lib_ruby_parser_nodes::{Field, FieldType, Node};

pub struct Visitor<'a> {
    nodes: &'a [Node],
}

impl<'a> Visitor<'a> {
    pub fn new(nodes: &'a [Node]) -> Self {
        Self { nodes }
    }

    pub fn write(&self) {
        std::fs::write("src/traverse/visitor/visit_gen.rs", self.contents()).unwrap()
    }

    fn contents(&self) -> String {
        format!(
            "use crate::nodes::*;
use crate::traverse::visitor::{{Item, Visit, Visitor}};
use crate::Node;

pub trait Observer {{
    {observer_methods}

    #[allow(unused_variables)]
    fn on_node(&mut self, node: &Node) {{}}
    #[allow(unused_variables)]
    fn on_node_moving_up(&mut self, node: &Node) {{}}

    #[allow(unused_variables)]
    fn on_option_node(&mut self, node: &Option<Box<Node>>) {{}}
    #[allow(unused_variables)]
    fn on_node_list(&mut self, nodes: &[Node]) {{}}

    #[allow(unused_variables)]
    fn on_subitem(&mut self, subitem: Item) {{}}
    #[allow(unused_variables)]
    fn on_subitem_moving_up(&mut self, subitem: Item) {{}}
}}

impl<TObserver: Observer> Visit<&Node> for Visitor<TObserver> {{
    fn visit(&mut self, node: &Node, visit_as: Item) {{
        self.handler.on_subitem(visit_as);
        self.handler.on_node(node);

        match node {{
            {match_branches}
        }}

        self.handler.on_node_moving_up(&node);
        self.handler.on_subitem_moving_up(visit_as);
    }}
}}

impl<T> Visitor<T>
where
    T: Observer,
{{
    {visitor_methods}
}}
",
            observer_methods = self.observer_methods().join("\n    "),
            match_branches = self.match_branches().join("\n            "),
            visitor_methods = self.visitor_methods().join("\n\n    ")
        )
    }

    fn observer_methods(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "#[allow(unused_variables)]
    fn on_{mid}(&mut self, node: &{struct_name}) {{}}",
                    mid = node.filename,
                    struct_name = node.struct_name
                )
            })
            .collect()
    }

    fn match_branches(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "Node::{struct_name}(inner) => self.visit_{mid}(inner),",
                    struct_name = node.struct_name,
                    mid = node.filename
                )
            })
            .collect()
    }
    fn visitor_methods(&self) -> Vec<String> {
        self.nodes
            .iter()
            .map(|node| {
                format!(
                    "fn visit_{mid}(&mut self, node: &{struct_name}) {{
        self.handler.on_{mid}(node);

        {visit_children}
    }}",
                    struct_name = node.struct_name,
                    mid = node.filename,
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
    pub fn new(node: &'a Node) -> Self {
        Self { node }
    }

    pub fn visit_children(&self) -> Vec<String> {
        self.node
            .fields
            .iter()
            .filter_map(|f| {
                match f.field_type {
                    FieldType::Node => {}
                    FieldType::Nodes => {}
                    FieldType::MaybeNode => {}
                    FieldType::RegexOptions => {}

                    FieldType::Loc
                    | FieldType::MaybeLoc
                    | FieldType::Str
                    | FieldType::MaybeStr
                    | FieldType::Chars
                    | FieldType::StringValue
                    | FieldType::U8
                    | FieldType::Usize
                    | FieldType::RawString => return None,
                }

                let variant = field_name_to_variant(self.node, f);

                Some(format!(
                    "self.visit(&node.{field_name}, Item::{variant});",
                    field_name = f.field_name,
                    variant = variant
                ))
            })
            .collect()
    }
}

fn field_name_to_variant(node: &Node, field: &Field) -> String {
    match (&node.str_type[..], &field.field_name[..]) {
        (_, "statements") => return "Stmts".to_owned(),
        (_, "call") => return "MethodCall".to_owned(),
        (_, "default") => return "DefaultValue".to_owned(),
        (_, "items") => return "MlhsItems".to_owned(),
        ("when", "patterns") => return "Args".to_owned(),
        ("undef", "names") => return "Args".to_owned(),
        ("args", "args") => return "Arglist".to_owned(),
        ("procarg0", "args") => return "Arglist".to_owned(),
        ("rescue", "else_") => return "ElseBody".to_owned(),
        // (_, "when_bodies") => return "WhenBodies".to_owned(),
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
