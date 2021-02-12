use crate::traverse::finder::PatternError;
use crate::traverse::visitor::Item as VisitorItem;

#[derive(Clone, Debug)]
pub enum Item {
    Any,
    Node(String),
    VisitorItem(VisitorItem),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Any, _) | (_, Item::Any) => true,
            (Item::Node(left), Item::Node(right)) => left.eq(right),
            (Item::VisitorItem(left), Item::VisitorItem(right)) => left.eq(right),
            (_, _) => false,
        }
    }
}

impl Eq for Item {}

impl Item {
    pub fn new(s: &str) -> Result<Self, PatternError> {
        match s {
            "any" => Ok(Self::Any),
            visitor_item if VisitorItem::from_string(visitor_item).is_ok() => {
                let visitor_item = VisitorItem::from_string(visitor_item).unwrap();
                Ok(Item::VisitorItem(visitor_item))
            }
            node_name => Ok(Self::Node(node_name.to_owned())),
        }
    }
}
