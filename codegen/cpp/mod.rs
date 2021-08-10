#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod helpers;

#[cfg(feature = "compile-with-external-structures")]
mod messages_cpp;
#[cfg(feature = "compile-with-external-structures")]
mod messages_hpp;

#[cfg(feature = "compile-with-external-structures")]
pub(crate) fn codegen() {
    messages_hpp::codegen();
    messages_cpp::codegen();
}

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
