mod item;
pub use item::Item;

mod error;
pub use error::PatternError;

#[derive(Debug, PartialEq, Eq)]
pub struct Pattern {
    parts: Vec<Item>,
}

impl Pattern {
    pub fn new(input: &str) -> Result<Self, PatternError> {
        let mut parts: Vec<Item> = vec![];

        for part in input.split(" -> ") {
            let part = Item::new(part)?;
            parts.push(part);
        }

        Ok(Self { parts })
    }

    pub fn empty() -> Self {
        Self { parts: vec![] }
    }

    pub fn push(&mut self, item: Item) {
        self.parts.push(item)
    }

    pub fn pop(&mut self) -> Option<Item> {
        self.parts.pop()
    }
}
