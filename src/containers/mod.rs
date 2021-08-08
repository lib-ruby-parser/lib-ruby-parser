pub(crate) mod list;
pub(crate) mod maybe_loc;
pub(crate) mod maybe_ptr;
pub(crate) mod maybe_string_ptr;
pub(crate) mod ptr;
pub(crate) mod shared_byte_list;
pub(crate) mod string_ptr;

pub(crate) mod helpers {
    pub(crate) use super::{
        list::TakeFirst as ListTakeFirst,
        maybe_loc::MaybeLocAPI,
        maybe_ptr::{MaybePtrNone, MaybePtrSome},
        maybe_string_ptr::MaybeStringPtrAPI,
        ptr::UnPtr,
    };
}

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod get_drop_fn;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod size;

#[cfg(feature = "compile-with-external-structures")]
pub use list::external::List as ExternalList;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use list::external::ListBlob;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_loc::external::MaybeLoc as ExternalMaybeLoc;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use maybe_loc::external::MaybeLocBlob;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_ptr::external::MaybePtr as ExternalMaybePtr;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use maybe_ptr::external::MaybePtrBlob;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_string_ptr::external::MaybeStringPtr as ExternalMaybeStringPtr;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use maybe_string_ptr::external::MaybeStringPtrBlob;
#[cfg(feature = "compile-with-external-structures")]
pub use ptr::external::Ptr as ExternalPtr;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) use ptr::external::PtrBlob;
#[cfg(feature = "compile-with-external-structures")]
pub use shared_byte_list::external::SharedByteList as ExternalSharedByteList;
#[cfg(feature = "compile-with-external-structures")]
pub use string_ptr::external::StringPtr as ExternalStringPtr;
