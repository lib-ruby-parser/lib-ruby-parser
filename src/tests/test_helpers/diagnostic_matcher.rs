use crate::Diagnostic;

pub fn render_diagnostic_for_testing<W: core::fmt::Write>(
    d: &Diagnostic,
    w: &mut W,
) -> core::fmt::Result {
    for _ in 0..d.loc.begin() {
        write!(w, " ")?;
    }
    for _ in 0..d.loc.size() {
        write!(w, "~")?;
    }
    write!(w, " ({}) ", d.level)?;
    d.render_message(w)?;
    Ok(())
}
