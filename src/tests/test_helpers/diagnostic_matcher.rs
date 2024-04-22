use crate::Diagnostic;

pub fn render_diagnostic_for_testing(d: &Diagnostic) -> String {
    let first = format!(
        "{prefix}{highlight} ({level}) ",
        prefix = " ".repeat(d.loc.begin),
        highlight = "~".repeat(d.loc.size()),
        level = d.level.to_string(),
    );
    let mut second = String::new();
    d.render_message(&mut second).unwrap();
    first + &second
}
