use lib_ruby_parser::Diagnostic;

pub fn render_diagnostic_for_testing(d: &Diagnostic) -> String {
    format!(
        "{prefix}{highlight} ({level}) {message}",
        prefix = " ".repeat(d.range.begin_pos),
        highlight = "~".repeat(d.range.size()),
        level = d.level.to_string(),
        message = d.render_message()
    )
}
