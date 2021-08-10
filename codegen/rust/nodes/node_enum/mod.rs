#[cfg(feature = "compile-with-external-structures")]
mod external;
#[cfg(not(feature = "compile-with-external-structures"))]
mod native;

pub(crate) fn codegen() {
    #[cfg(feature = "compile-with-external-structures")]
    external::codegen();

    #[cfg(not(feature = "compile-with-external-structures"))]
    native::codegen();
}
