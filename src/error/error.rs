use crate::source::buffer::*;
use crate::source::Range;
use crate::{ErrorLevel, ErrorMessage};

#[derive(Debug, Clone)]
pub struct ParseError {
    level: ErrorLevel,
    message: ErrorMessage,
    range: Range,
}

impl ParseError {
    pub fn new(level: ErrorLevel, message: ErrorMessage, range: Range) -> Self {
        Self {
            level,
            message,
            range,
        }
    }

    pub fn render_message(&self) -> String {
        self.message.render()
    }

    pub fn render(&self, buffer: &Buffer) -> Option<String> {
        let (start_loc, end_loc) = self.range.to_locs(&buffer)?;
        debug_assert!(start_loc.line == end_loc.line, "multi-line error");
        let line_no = start_loc.line;
        let line = buffer.lines[line_no - 1]
            .source(&buffer.input)
            .iter()
            .collect::<String>();

        Some(format!(
            "{:#?}: {:#?}: {}\n{}:{}: {}\n{}:{}: {}",
            start_loc,
            self.level,
            self.message.render(),
            buffer.name,
            line_no,
            line,
            buffer.name,
            line_no,
            "^~~"
        ))
    }
}
