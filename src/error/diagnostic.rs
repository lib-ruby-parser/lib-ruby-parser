use core::{cell::Cell, ptr::NonNull};

use lib_ruby_parser_ast_arena::{Blob, DiagnosticMessage};

use crate::loc_ext::LocExt;
use crate::source::DecodedInput;
use crate::ErrorLevel;
use crate::Loc;

/// Diagnostic message that comes from the parser when there's an error or warning
#[repr(C)]
pub struct Diagnostic<'b> {
    /// Level of the diagnostic (error or warnings)
    pub level: ErrorLevel,

    /// Message of the diagnostic
    pub message: DiagnosticMessage<'b>,

    /// Location of the diagnostic
    pub loc: Loc,

    next: Cell<Option<NonNull<Self>>>,

    blob: &'b Blob<'b>,
}

impl core::fmt::Debug for Diagnostic<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Diagnostic")
            .field("level", &self.level)
            .field("message", &self.message)
            .field("loc", &self.loc)
            .finish()
    }
}

impl PartialEq for Diagnostic<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.level == other.level && self.message == other.message && self.loc == other.loc
    }
}

impl<'b> Diagnostic<'b> {
    pub(crate) fn new(
        level: ErrorLevel,
        message: DiagnosticMessage<'b>,
        loc: Loc,
        blob: &'b Blob<'b>,
    ) -> &'b Self {
        let this = blob.alloc_mut();
        *this = Self {
            level,
            message,
            loc,
            next: Cell::new(None),
            blob,
        };
        this
    }

    /// Returns rendered message
    pub fn render_message(&self) -> String {
        let mut out = String::new();
        self.message.render(&mut out).unwrap();
        out
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
                message = self.render_message(),
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

impl lib_ruby_parser_ast_arena::IntrusiveListItem for Diagnostic<'_> {
    fn next(&self) -> Option<NonNull<Self>> {
        self.next.get()
    }

    fn set_next(&self, new_next: NonNull<Self>) {
        self.next.set(Some(new_next))
    }
}

#[test]
fn test_renders() {
    let source = "line 1\nvery long line 2\n";
    let mut mem = vec![0; 1000];
    let blob = Blob::from(mem.as_mut_slice());
    let input = DecodedInput::new("(test_render)", source.as_bytes(), &blob);

    let error = Diagnostic::new(
        ErrorLevel::Warning,
        DiagnosticMessage::FractionAfterNumeric {},
        Loc { begin: 8, end: 12 },
        &blob,
    );

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
    let mut mem = vec![0; 1000];
    let blob = Blob::from(mem.as_mut_slice());

    let error = Diagnostic::new(
        ErrorLevel::Error,
        DiagnosticMessage::AliasNthRef {},
        Loc { begin: 1, end: 2 },
        &blob,
    );

    let warning = Diagnostic::new(
        ErrorLevel::Warning,
        DiagnosticMessage::AliasNthRef {},
        Loc { begin: 1, end: 2 },
        &blob,
    );

    assert!(error.is_error());
    assert!(!error.is_warning());

    assert!(!warning.is_error());
    assert!(warning.is_warning());
}
