#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible nullable pointer
    pub type MaybePtr<T> = Option<Box<T>>;
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible nullable pointer
    #[derive(Debug)]
    #[repr(C)]
    pub struct MaybePtr<T> {
        ptr: *mut T,
    }
}
