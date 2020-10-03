use crate::{ErrorLevel, ErrorMessage};
use crate::source::Range;

#[derive(Debug, Clone)]
pub struct ParseError {
    level: ErrorLevel,
    message: ErrorMessage,
    range: Range,
}

impl ParseError {
    pub fn new(level: ErrorLevel, message: ErrorMessage, range: Range) -> Self {
        Self { level, message, range }
    }

    pub fn render_message(&self) -> String {
        self.message.render()
    }
}
