use crate::source::{buffer::Input, Range};
use crate::{DiagnosticMessage, ErrorLevel};

/// Diagnostic message that comes from the parser when there's an error or warning
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: ErrorLevel,
    pub message: DiagnosticMessage,
    pub range: Range,
}

impl Diagnostic {
    pub fn new(level: ErrorLevel, message: DiagnosticMessage, range: Range) -> Self {
        Self {
            level,
            message,
            range,
        }
    }

    pub fn render_message(&self) -> String {
        self.message.render()
    }

    pub fn render(&self, input: &Input) -> Option<String> {
        let (line_no, line_loc) = self.range.expand_to_line(input)?;
        let line = line_loc.source(input)?;

        let filename = &input.name;
        let (_, start_col) = self.range.begin_line_col(input)?;

        let prefix = format!("{}:{}", filename, line_no + 1);
        let highlight = format!(
            "{indent}^{tildes}",
            indent = " ".repeat(start_col),
            tildes = if self.range.size() > 0 {
                "~".repeat(self.range.size() - 1)
            } else {
                "".to_owned()
            }
        );

        Some(
            format!(
                "{prefix}:{start_col}: {level:?}: {message}\n{prefix}: {line}\n{prefix}: {highlight}",
                prefix = prefix,
                start_col = start_col,
                level = self.level,
                message = self.message.render(),
                line = line,
                highlight = highlight
            )
            .trim()
            .to_owned(),
        )
    }

    pub fn is_warning(&self) -> bool {
        matches!(self.level, ErrorLevel::Warning)
    }

    pub fn is_error(&self) -> bool {
        matches!(self.level, ErrorLevel::Error)
    }
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub(crate) struct Diagnostics {
    list: Rc<RefCell<Vec<Diagnostic>>>,
}

impl Diagnostics {
    pub(crate) fn emit(&self, diagnostic: Diagnostic) {
        self.list.borrow_mut().push(diagnostic)
    }

    pub(crate) fn take_inner(self) -> Vec<Diagnostic> {
        self.list.replace(vec![])
    }
}
