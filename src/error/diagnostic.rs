use crate::source::buffer::Input;
use crate::Loc;
use crate::{DiagnosticMessage, ErrorLevel};

/// Diagnostic message that comes from the parser when there's an error or warning
#[derive(Debug, Clone)]
pub struct Diagnostic {
    pub level: ErrorLevel,
    pub message: DiagnosticMessage,
    pub loc: Loc,
}

impl Diagnostic {
    pub fn new(level: ErrorLevel, message: DiagnosticMessage, loc: Loc) -> Self {
        Self {
            level,
            message,
            loc,
        }
    }

    pub fn render_message(&self) -> String {
        self.message.render()
    }

    pub fn render(&self, input: &Input) -> Option<String> {
        let (line_no, line_loc) = self.loc.expand_to_line(input)?;
        let line = line_loc.source(input)?;

        let filename = &input.name;
        let (_, start_col) = self.loc.begin_line_col(input)?;

        let prefix = format!("{}:{}", filename, line_no + 1);
        let highlight = format!(
            "{indent}^{tildes}",
            indent = " ".repeat(start_col),
            tildes = if self.loc.size() > 0 {
                "~".repeat(self.loc.size() - 1)
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
    pub(crate) fn new() -> Self {
        Self {
            list: Rc::new(RefCell::new(vec![])),
        }
    }

    pub(crate) fn emit(&self, diagnostic: Diagnostic) {
        self.list.borrow_mut().push(diagnostic)
    }

    pub(crate) fn take_inner(self) -> Vec<Diagnostic> {
        self.list.replace(vec![])
    }
}
