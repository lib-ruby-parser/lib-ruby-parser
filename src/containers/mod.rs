pub(crate) mod list;
pub(crate) mod maybe;
pub(crate) mod ptr;
pub(crate) mod shared_byte_list;
pub(crate) mod string_ptr;

/// Module with compatibility APIs that are implemented for both native AND external containers
pub mod helpers {
    pub use super::{list::ListAPI, maybe::MaybeAPI, ptr::PtrAPI};
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod size;

#[cfg(feature = "compile-with-external-structures")]
pub use list::external::List as ExternalList;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use list::external::{list, list_count};
#[cfg(feature = "compile-with-external-structures")]
pub use maybe::external::Maybe as ExternalMaybe;
#[cfg(feature = "compile-with-external-structures")]
pub use ptr::external::Ptr as ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
pub use shared_byte_list::external::SharedByteList as ExternalSharedByteList;
#[cfg(feature = "compile-with-external-structures")]
pub use string_ptr::external::StringPtr as ExternalStringPtr;

mod use_native_or_external;
pub(crate) use use_native_or_external::use_native_or_external;
