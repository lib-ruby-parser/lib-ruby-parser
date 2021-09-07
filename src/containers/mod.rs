pub(crate) mod list;
pub(crate) mod maybe_loc;
pub(crate) mod maybe_ptr;
pub(crate) mod maybe_string_ptr;
pub(crate) mod ptr;
pub(crate) mod shared_byte_list;
pub(crate) mod string_ptr;

pub(crate) mod helpers {
    pub(crate) use super::{
        list::ListAPI, maybe_loc::MaybeLocAPI, maybe_ptr::MaybePtrAPI,
        maybe_string_ptr::MaybeStringPtrAPI, ptr::PtrAPI,
    };
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod get_drop_fn;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod size;

#[cfg(feature = "compile-with-external-structures")]
pub use list::external::List as ExternalList;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_loc::external::MaybeLoc as ExternalMaybeLoc;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_ptr::external::MaybePtr as ExternalMaybePtr;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_string_ptr::external::MaybeStringPtr as ExternalMaybeStringPtr;
#[cfg(feature = "compile-with-external-structures")]
pub use ptr::external::Ptr as ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
pub use shared_byte_list::external::SharedByteList as ExternalSharedByteList;
#[cfg(feature = "compile-with-external-structures")]
pub use string_ptr::external::StringPtr as ExternalStringPtr;

mod use_native_or_external;
pub(crate) use use_native_or_external::use_native_or_external;
