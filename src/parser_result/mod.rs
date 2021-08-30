#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::ParserResult;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::ParserResult;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::ParserResultBlob;

mod shared;

#[cfg(test)]
mod tests;
