use crate::containers::LocPtr;
use crate::source::Input;
use crate::{DiagnosticMessage, ErrorLevel};

/// Diagnostic message that comes from the parser when there's an error or warning
#[derive(Debug, Clone)]
pub struct Diagnostic {
    /// Level of the diagnostic (error or warnings)
    pub level: ErrorLevel,
    /// Message of the diagnostic
    pub message: DiagnosticMessage,
    /// Location of the diagnostic
    pub loc: LocPtr,
}

impl Diagnostic {
    /// Construncts an instance of `Diagnostic`
    pub fn new(level: ErrorLevel, message: DiagnosticMessage, loc: LocPtr) -> Self {
        Self {
            level,
            message,
            loc,
        }
    }

    /// Returns rendered message
    pub fn render_message(&self) -> String {
        self.message.render()
    }

    /// Renders all data into a single String, produces an output like:
    ///
    /// ```text
    /// (test.rb):1:5: error: unexpected END_OF_INPUT
    /// (test.rb):1: foo++
    /// (test.rb):1:      ^
    /// ```
    pub fn render(&self, input: &Input) -> Option<String> {
        println!("input = {:?}", input);
        let x = self.loc.expand_to_line(input);
        println!("x = {:?}", x);
        let (line_no, line_loc) = x?;
        let line = line_loc.source(input)?;

        let filename = &input.name;
        let (_, start_col) = self.loc.begin_line_col(input)?;

        let prefix = format!("{}:{}", filename.to_string_lossy(), line_no + 1);
        let highlight = format!(
            "{indent}^{tildes}",
            indent = " ".repeat(start_col),
            tildes = if self.loc.size() > 0 {
                "~".repeat(self.loc.size() - 1)
            } else {
                "".to_string()
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
            .to_string(),
        )
    }

    /// Returns `true` if level of the diagnostic is `Warning`
    pub fn is_warning(&self) -> bool {
        matches!(self.level, ErrorLevel::Warning)
    }

    /// Returns `true` if level of the diagnostic is `Error`
    pub fn is_error(&self) -> bool {
        matches!(self.level, ErrorLevel::Error)
    }
}

use crate::containers::List;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub(crate) struct Diagnostics {
    list: Rc<RefCell<List<Diagnostic>>>,
}

impl Diagnostics {
    pub(crate) fn new() -> Self {
        Self {
            list: Rc::new(RefCell::new(List::new())),
        }
    }

    pub(crate) fn emit(&self, diagnostic: Diagnostic) {
        self.list.borrow_mut().push(diagnostic)
    }

    pub(crate) fn take_inner(self) -> List<Diagnostic> {
        self.list.replace(List::new())
    }
}
