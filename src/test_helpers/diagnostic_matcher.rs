use crate::Diagnostic;

pub fn render_diagnostic_for_testing(d: &Diagnostic) -> String {
    format!(
        "{prefix}{highlight} ({level}) {message}",
        prefix = " ".repeat(d.loc().begin()),
        highlight = "~".repeat(d.loc().size()),
        level = d.level().to_string(),
        message = d.render_message()
    )
}
