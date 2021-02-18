mod diagnostic;
mod level;
mod message;
mod message_gen;

pub use diagnostic::Diagnostic;
pub(crate) use diagnostic::Diagnostics;
pub use level::ErrorLevel;
pub use message_gen::DiagnosticMessage;
