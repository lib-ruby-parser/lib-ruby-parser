#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::SourceLine;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::SourceLine;
#[cfg(feature = "compile-with-external-structures")]
#[allow(unused_imports)]
pub(crate) use external::SourceLineBlob;

mod shared;

#[cfg(test)]
mod tests;
