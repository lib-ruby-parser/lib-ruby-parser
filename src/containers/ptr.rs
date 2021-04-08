#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible not-null pointer
    pub type Ptr<T> = Box<T>;
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible not-null pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct Ptr<T> {
        ptr: *mut T,
    }
}
