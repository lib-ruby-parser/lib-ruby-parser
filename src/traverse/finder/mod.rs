mod pattern;
pub use pattern::{Item as PatternItem, Pattern, PatternError};

use crate::traverse::visitor::{Item as VisitorItem, Observer, Visitor};
use crate::Node;

pub struct Finder {
    looking_for: Pattern,
    current_path: Pattern,
    result: Option<Node>,
}

impl Finder {
    pub fn run(looking_for: &str, root: &Node) -> Result<Option<Node>, PatternError> {
        let looking_for = Pattern::new(looking_for)?;
        let mut visitor = Visitor {
            handler: Self {
                looking_for,
                current_path: Pattern::empty(),
                result: None,
            },
        };
        visitor.visit_root(&root);
        Ok(visitor.handler.result)
    }
}

impl Observer for Finder {
    fn on_node(&mut self, node: &Node) {
        if self.current_path == self.looking_for {
            self.result = Some(node.clone());
        }
    }

    fn on_subitem(&mut self, subitem: VisitorItem) {
        self.current_path.push(PatternItem::VisitorItem(subitem))
    }

    fn on_subitem_moving_up(&mut self, _: VisitorItem) {
        self.current_path.pop().unwrap();
    }
}
