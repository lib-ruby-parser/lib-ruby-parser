#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::ParserOptions;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::ParserOptions;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::ParserOptionsBlob;

mod internal;
pub(crate) use internal::InternalParserOptions;

mod shared;

#[cfg(test)]
mod tests;
