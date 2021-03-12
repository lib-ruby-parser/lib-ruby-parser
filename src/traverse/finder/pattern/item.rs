use crate::traverse::finder::PatternError;
use crate::traverse::visitor::Item as VisitorItem;

/// Finder item, used to specify part of the path you want to go in AST
#[derive(Clone, Debug)]
pub enum Item {
    /// Represents a wildcard pattern item
    Any,
    /// Represents a specific `visitor::Item` for matching
    VisitorItem(VisitorItem),
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Item::Any, _) | (_, Item::Any) => true,
            (Item::VisitorItem(left), Item::VisitorItem(right)) => left.eq(right),
        }
    }
}

impl Eq for Item {}

impl Item {
    /// Parses given string slice and constructs an `Item` (if possible)
    pub fn new(s: &str) -> Result<Self, PatternError> {
        match s {
            "any" => Ok(Self::Any),
            visitor_item if VisitorItem::from_string(visitor_item).is_ok() => {
                let visitor_item = VisitorItem::from_string(visitor_item).unwrap();
                Ok(Item::VisitorItem(visitor_item))
            }
            unknown => Err(PatternError {
                pattern: unknown.to_string(),
            }),
        }
    }
}
