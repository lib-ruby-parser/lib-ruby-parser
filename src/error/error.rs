use crate::{ErrorLevel, ErrorMessage};
use crate::source::Range;
use crate::Buffer;

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

    pub fn render(&self, buffer: &Buffer) -> Option<String> {
        let (start_loc, end_loc) = self.range.to_locs(&buffer)?;
        debug_assert!(start_loc.line == end_loc.line, "multi-line error");
        println!("{:#?} - {:#?}", start_loc, end_loc);
        let line_no = start_loc.line;
        let line = String::from_utf8_lossy(&buffer.lines[line_no - 1].source(&buffer.input)).into_owned();

        Some(
            format!("{:#?}: {:#?}: {}\n{}:{}: {}\n{}:{}: {}",
                start_loc, self.level, self.message.render(),
                buffer.name, line_no, line,
                buffer.name, line_no, "^~~")
        )
    }
}
