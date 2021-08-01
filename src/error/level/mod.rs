#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::ErrorLevel;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::ErrorLevel;
#[cfg(feature = "compile-with-external-structures")]
#[allow(unused_imports)]
pub(crate) use external::ErrorLevelBlob;

mod shared;

#[cfg(test)]
mod tests;
