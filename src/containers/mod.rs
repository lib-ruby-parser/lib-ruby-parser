/// Module with List container
pub mod list;

/// Module with MaybePtr container
pub mod maybe_ptr;

/// Module with Ptr container
pub mod ptr;

/// Module with Loc container
pub mod loc;

/// Module with MaybeLoc container
pub mod maybe_loc;

/// Module with StringPtr container
pub mod string_ptr;

/// Module with MaybeStringPtr container
pub mod maybe_string_ptr;

/// Module with SharedByteList container
pub mod shared_byte_list;

#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod get_drop_fn;
#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod size;

#[cfg(not(feature = "compile-with-external-structures"))]
pub use list::rust::List;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use loc::rust::Loc;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use maybe_loc::rust::MaybeLoc;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use maybe_ptr::rust::MaybePtr;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use maybe_string_ptr::rust::MaybeStringPtr;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use ptr::rust::Ptr;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use shared_byte_list::rust::SharedByteList;
#[cfg(not(feature = "compile-with-external-structures"))]
pub use string_ptr::rust::StringPtr;

#[cfg(feature = "compile-with-external-structures")]
pub use list::c::List;
#[cfg(feature = "compile-with-external-structures")]
pub use loc::c::Loc;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_loc::c::MaybeLoc;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_ptr::c::MaybePtr;
#[cfg(feature = "compile-with-external-structures")]
pub use maybe_string_ptr::c::MaybeStringPtr;
#[cfg(feature = "compile-with-external-structures")]
pub use ptr::c::Ptr;
#[cfg(feature = "compile-with-external-structures")]
pub use shared_byte_list::c::SharedByteList;
#[cfg(feature = "compile-with-external-structures")]
pub use string_ptr::c::StringPtr;
