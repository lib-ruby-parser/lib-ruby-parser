mod item;
pub use item::Item;

mod error;
pub use error::PatternError;

/// Pattern that is used for matching.
///
/// Consists of multiple `finder::Item` elements
///
/// For example the following pattern:
///
/// ```text
/// Pattern::new("args -> arglist -> 2 -> default")
/// ```
///
/// can find a node that represents constant `FIND_ME` in the following code:
///
/// ```text
/// def foo(a, b, c = FIND_ME)
/// end
/// ```
///
/// It means:
/// 1. enter `.args` of the `Def` node (`(a, b, c = FIND_ME`))
/// 2. enter its `.argslist` (`a, b, c = FIND_ME`)
/// 3. enter element `[2]` (`c = FIND_ME`)
/// 4. enter `.default` of the `Optarg` node (`FIND_ME`)
#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    pub(crate) parts: Vec<Item>,
}

impl Pattern {
    /// Constructs a pattern from a string, returns an error on the first sub-pattern error
    pub fn new(input: &str) -> Result<Self, PatternError> {
        let mut parts: Vec<Item> = vec![];

        for part in input.split(" -> ") {
            let part = Item::new(part)?;
            parts.push(part);
        }

        Ok(Self { parts })
    }

    /// Returns `true` if pattern is empty
    pub fn empty() -> Self {
        Self { parts: vec![] }
    }

    /// Pushes a new `Item` into a pattern
    pub fn push(&mut self, item: Item) {
        self.parts.push(item)
    }

    /// Pops an `Item` from a pattern
    pub fn pop(&mut self) -> Option<Item> {
        self.parts.pop()
    }

    pub(crate) fn unshift(&mut self) -> Option<Item> {
        if self.parts.is_empty() {
            None
        } else {
            Some(self.parts.remove(0))
        }
    }
}
