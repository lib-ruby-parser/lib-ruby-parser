use crate::source::Range;
use crate::{DiagnosticMessage, ErrorLevel};

#[derive(Debug, Clone)]
pub struct Diagnostic {
    level: ErrorLevel,
    message: DiagnosticMessage,
    range: Range,
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

    pub fn render(&self) -> Option<String> {
        let (line_no, line_loc) = self.range.expand_to_line()?;
        let line = line_loc.source()?;

        let filename = &self.range.input.name;
        let (start_col, _) = self.range.begin_line_col()?;

        let prefix = format!("{}:{}", filename, line_no + 1);

        Some(
            format!(
                "{prefix}:{start_col}: {level:?}: {message}\n{prefix}: {line}\n{prefix}: {highlight}",
                prefix = prefix,
                start_col = start_col,
                level = self.level,
                message = self.message.render(),
                line = line,
                highlight = "^~~"
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
