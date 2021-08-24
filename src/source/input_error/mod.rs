#[cfg(not(feature = "compile-with-external-structures"))]
mod native;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use native::InputError;

#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(feature = "compile-with-external-structures")]
pub use external::InputError;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use external::InputErrorBlob;

mod shared;

#[cfg(test)]
mod tests;
