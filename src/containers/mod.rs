mod list;
mod maybe_loc;
mod maybe_ptr;
mod maybe_string_ptr;
mod ptr;
mod shared_byte_list;
mod string_ptr;

pub(crate) mod helpers {
    pub(crate) use super::{
        list::TakeFirst as ListTakeFirst,
        maybe_ptr::{MaybePtrNone, MaybePtrSome},
        maybe_string_ptr::{MaybeStringPtrNone, MaybeStringPtrSome},
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
