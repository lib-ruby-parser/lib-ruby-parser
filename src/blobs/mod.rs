macro_rules! declare_blob {
    ($doc:literal, $name:ident, $size:expr) => {
        #[repr(C)]
        #[derive(Debug, Clone, Copy)]
        #[doc=$doc]
        pub struct $name {
            pub(crate) bytes: [u8; $size],
        }
    };
}

mod gen;
pub use gen::*;
