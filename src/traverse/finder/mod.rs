mod pattern;
pub use pattern::{Item as PatternItem, Pattern, PatternError};

use crate::traverse::visitor::Visitor;
use crate::Node;

/// A struct to find sub-nodes in AST by by a given `Pattern`
#[derive(Debug)]
pub struct Finder {
    pattern: Pattern,
    result: Option<Node>,
}

mod finder_gen;

impl Finder {
    /// Performs a search of a given pattern on a given AST.
    ///
    /// `pattern` is a string slice that is used to construct a `Pattern`.
    pub fn run(pattern: &str, root: &Node) -> Result<Option<Node>, PatternError> {
        let mut pattern = Pattern::new(pattern)?;
        println!("{:?}", pattern);
        debug_assert_eq!(pattern.unshift(), Some(PatternItem::Root));
        let mut finder = Self {
            pattern,
            result: None,
        };
        finder.visit(root);
        Ok(finder.result)
    }
}

#[cfg(test)]
mod tests;
