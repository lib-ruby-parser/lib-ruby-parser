#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::Loc;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::Loc;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::LocBlob;

mod shared;

#[cfg(test)]
mod tests;
