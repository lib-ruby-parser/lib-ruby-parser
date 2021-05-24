mod item;
pub use item::Item;

mod visit_gen;
pub use visit_gen::Observer;

use crate::{
    containers::{maybe_ptr::AsOption, MaybePtr, Ptr},
    Node,
};

#[cfg(feature = "compile-with-external-structures")]
use crate::containers::ExternalList;
#[cfg(feature = "compile-with-external-structures")]
type List<T> = ExternalList<T>;
#[cfg(not(feature = "compile-with-external-structures"))]
type List<T> = Vec<T>;

/// Generic visitor of `Node`.
///
/// It doesn't do anything on its own,
/// but it notifies given `Observer`.
///
/// ```text
/// struct MyObserver {
///     pub numbers: Vec<nodes::Int>,
/// }
///
/// impl Observer for MyObserver {
///     fn on_int(&mut self, node: &nodes::Int) {
///         self.numbers.push(node.clone())
///     }
/// }
///
/// let source = "2 + 3";
/// let mut parser = Parser::new(source.as_bytes(), ParserOptions::default());
/// let ast = parser.do_parse().ast.unwrap();
///
/// let observer = MyObserver { numbers: vec![] };
/// let visitor = Visitor { observer };
///
/// visitor.visit_root(&ast);
///
/// println!("{:?}", visitor.observer.numbers);
/// // => [Int { value: "2" }, Int { value: "3" }]
/// ```
#[derive(Debug)]
pub struct Visitor<T>
where
    T: Observer,
{
    /// Observer of the visitor, receives calls like `on_int(&mut self, node: nodes::Int)`
    pub observer: T,
}

pub(crate) trait Visit<TItem> {
    fn visit(&mut self, item: TItem, visit_as: Item);
}

impl<TObserver: Observer> Visit<&[Node]> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &[Node], visit_as: Item) {
        self.observer.on_subitem(visit_as);
        self.observer.on_node_list(nodes);

        for (idx, node) in nodes.iter().enumerate() {
            self.visit(node, Item::Idx(idx));
        }

        self.observer.on_subitem_moving_up(visit_as);
    }
}

impl<TObserver: Observer> Visit<&List<Node>> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &List<Node>, visit_as: Item) {
        let nodes: &[Node] = nodes;
        self.visit(nodes, visit_as);
    }
}

impl<TObserver: Observer> Visit<&Ptr<Node>> for Visitor<TObserver> {
    fn visit(&mut self, node: &Ptr<Node>, visit_as: Item) {
        let node: &Node = &*node;
        self.visit(node, visit_as);
    }
}

impl<TObserver: Observer> Visit<&MaybePtr<Node>> for Visitor<TObserver> {
    fn visit(&mut self, node: &MaybePtr<Node>, visit_as: Item) {
        if let Some(node) = node.as_option() {
            self.visit(node, visit_as);
        }
    }
}

impl<T> Visitor<T>
where
    T: Observer,
{
    /// Starts traversing on a given `node`
    pub fn visit_root(&mut self, node: &Node) {
        self.visit(node, Item::Root);
    }
}
