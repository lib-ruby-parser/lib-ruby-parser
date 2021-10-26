use super::Diagnostic;
use crate::source::DecodedInput;
use std::cell::RefCell;
use std::rc::Rc;

impl Diagnostic {
    /// Returns rendered message
    pub fn render_message(&self) -> String {
        self.message().render()
    }

    /// Renders all data into a single String, produces an output like:
    ///
    /// ```text
    /// (test.rb):1:5: error: unexpected END_OF_INPUT
    /// (test.rb):1: foo++
    /// (test.rb):1:      ^
    /// ```
    pub fn render(&self, input: &DecodedInput) -> Option<String> {
        let (line_no, line_loc) = self.loc().expand_to_line(input)?;
        let line = line_loc.source(input)?;

        let filename = &input.name();
        let (_, start_col) = self.loc().begin_line_col(input)?;

        let prefix = format!("{}:{}", filename.as_str(), line_no + 1);
        let highlight = format!(
            "{indent}^{tildes}",
            indent = " ".repeat(start_col),
            tildes = if self.loc().size() > 0 {
                "~".repeat(self.loc().size() - 1)
            } else {
                "".to_string()
            }
        );

        Some(
            format!(
                "{prefix}:{start_col}: {level:?}: {message}\n{prefix}: {line}\n{prefix}: {highlight}",
                prefix = prefix,
                start_col = start_col,
                level = self.level(),
                message = self.message().render(),
                line = line,
                highlight = highlight
            )
            .trim()
            .to_string(),
        )
    }

    /// Returns `true` if level of the diagnostic is `Warning`
    pub fn is_warning(&self) -> bool {
        self.level().is_warning()
    }

    /// Returns `true` if level of the diagnostic is `Error`
    pub fn is_error(&self) -> bool {
        self.level().is_error()
    }
}

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
