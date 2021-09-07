#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::variants;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::DiagnosticMessage;

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::variants;
#[cfg(feature = "compile-with-external-structures")]
pub use external::DiagnosticMessage;

mod render;

#[cfg(test)]
mod tests;
