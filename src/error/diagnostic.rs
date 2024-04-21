use crate::loc_ext::LocExt;
use crate::source::DecodedInput;
use crate::Loc;
use crate::{DiagnosticMessage, ErrorLevel};
use std::cell::RefCell;
use std::rc::Rc;

/// Diagnostic message that comes from the parser when there's an error or warning
#[derive(Debug, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Diagnostic {
    /// Level of the diagnostic (error or warnings)
    pub level: ErrorLevel,

    /// Message of the diagnostic
    pub message: DiagnosticMessage,

    /// Location of the diagnostic
    pub loc: Loc,
}

impl Diagnostic {
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
    pub fn render(&self, input: &DecodedInput) -> Option<String> {
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
                "".to_string()
            }
        );

        Some(
            format!(
                "{prefix}:{start_col}: {level}: {message}\n{prefix}: {line}\n{prefix}: {highlight}",
                prefix = prefix,
                start_col = start_col,
                level = self.level.to_string(),
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

#[test]
fn test_renders() {
    let source = "line 1\nvery long line 2\n";
    let mut mem = vec![0; 1000];
    let blob = lib_ruby_parser_ast_arena::Blob::from(mem.as_mut_slice());
    let input = crate::source::DecodedInput::new("(test_render)", source.as_bytes(), &blob);

    let error = Diagnostic {
        level: ErrorLevel::Warning,
        message: DiagnosticMessage::FractionAfterNumeric {},
        loc: Loc { begin: 8, end: 12 },
    };

    assert_eq!(
        error.render(&input).expect("failed to render diagnostic"),
        vec![
            "(test_render):2:1: warning: unexpected fraction part after numeric literal",
            "(test_render):2: very long line 2",
            "(test_render):2:  ^~~~"
        ]
        .join("\n")
    );
}

#[test]
fn test_predicates() {
    let error = Diagnostic {
        level: ErrorLevel::Error,
        message: DiagnosticMessage::AliasNthRef {},
        loc: Loc { begin: 1, end: 2 },
    };

    let warning = Diagnostic {
        level: ErrorLevel::Warning,
        message: DiagnosticMessage::AliasNthRef {},
        loc: Loc { begin: 1, end: 2 },
    };

    assert!(error.is_error());
    assert!(!error.is_warning());

    assert!(!warning.is_error());
    assert!(warning.is_warning());
}
