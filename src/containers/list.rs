#[cfg(not(feature = "c-structures"))]
pub mod rust {
    /// Rust-compatible list
    pub type List<T> = Vec<T>;
}

#[cfg(feature = "c-structures")]
pub mod c {
    /// C-compatible list
    #[derive(Debug)]
    #[repr(C)]
    pub struct List<T> {
        ptr: *mut T,
        len: usize,
        capacity: usize,
    }
}
