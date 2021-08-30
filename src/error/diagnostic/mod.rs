#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::Diagnostic;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::Diagnostic;

mod shared;
pub(crate) use shared::Diagnostics;

#[cfg(test)]
mod tests;
