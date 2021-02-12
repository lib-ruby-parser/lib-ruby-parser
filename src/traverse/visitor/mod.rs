mod item;
pub use item::Item;

mod visit_gen;
pub use visit_gen::Observer;

use crate::Node;

pub struct Visitor<T>
where
    T: Observer,
{
    pub handler: T,
}

pub(crate) trait Visit<TItem> {
    fn visit(&mut self, item: TItem, visit_as: Item);
}

impl<TObserver: Observer> Visit<&[Node]> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &[Node], visit_as: Item) {
        self.handler.on_subitem(visit_as);
        self.handler.on_node_list(nodes);

        for (idx, node) in nodes.iter().enumerate() {
            self.visit(node, Item::Idx(idx));
        }

        self.handler.on_subitem_moving_up(visit_as);
    }
}

impl<TObserver: Observer> Visit<&Vec<Node>> for Visitor<TObserver> {
    fn visit(&mut self, nodes: &Vec<Node>, visit_as: Item) {
        let nodes: &[Node] = nodes;
        self.visit(nodes, visit_as);
    }
}

impl<TObserver: Observer> Visit<&Box<Node>> for Visitor<TObserver> {
    fn visit(&mut self, node: &Box<Node>, visit_as: Item) {
        let node: &Node = &*node;
        self.visit(node, visit_as);
    }
}

impl<TObserver: Observer> Visit<&Option<Box<Node>>> for Visitor<TObserver> {
    fn visit(&mut self, node: &Option<Box<Node>>, visit_as: Item) {
        if let Some(node) = node {
            self.visit(node, visit_as);
        }
    }
}

impl<T> Visitor<T>
where
    T: Observer,
{
    pub fn visit_root(&mut self, node: &Node) {
        self.visit(node, Item::Root);
    }
}
