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

/// Module with SharedList container
pub mod shared_list;

#[cfg(not(feature = "c-structures"))]
pub use list::rust::List;
#[cfg(not(feature = "c-structures"))]
pub use loc::rust::Loc;
#[cfg(not(feature = "c-structures"))]
pub use maybe_loc::rust::MaybeLoc;
#[cfg(not(feature = "c-structures"))]
pub use maybe_ptr::rust::MaybePtr;
#[cfg(not(feature = "c-structures"))]
pub use maybe_string_ptr::rust::MaybeStringPtr;
#[cfg(not(feature = "c-structures"))]
pub use ptr::rust::Ptr;
#[cfg(not(feature = "c-structures"))]
pub use shared_list::rust::SharedList;
#[cfg(not(feature = "c-structures"))]
pub use string_ptr::rust::StringPtr;

#[cfg(feature = "c-structures")]
pub use list::c::List;
#[cfg(feature = "c-structures")]
pub use loc::c::Loc;
#[cfg(feature = "c-structures")]
pub use maybe_loc::c::MaybeLoc;
#[cfg(feature = "c-structures")]
pub use maybe_ptr::c::MaybePtr;
#[cfg(feature = "c-structures")]
pub use maybe_string_ptr::c::MaybeStringPtr;
#[cfg(feature = "c-structures")]
pub use ptr::c::Ptr;
#[cfg(feature = "c-structures")]
pub use shared_list::c::SharedList;
#[cfg(feature = "c-structures")]
pub use string_ptr::c::StringPtr;
