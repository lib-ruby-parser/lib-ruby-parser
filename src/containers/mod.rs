/// Module with List container
pub mod list;

/// Module with MaybePtr container
pub mod maybe_ptr;

/// Module with Ptr container
pub mod ptr;

#[cfg(not(feature = "c-structures"))]
pub use list::rust::List;
#[cfg(not(feature = "c-structures"))]
pub use maybe_ptr::rust::MaybePtr;
#[cfg(not(feature = "c-structures"))]
pub use ptr::rust::Ptr;

#[cfg(feature = "c-structures")]
pub use list::c::List;
#[cfg(feature = "c-structures")]
pub use maybe_ptr::c::MaybePtr;
#[cfg(feature = "c-structures")]
pub use ptr::c::Ptr;
