#[cfg(feature = "rebuild-grammar")]
pub(crate) mod helpers;

#[cfg(feature = "rebuild-grammar")]
mod messages_cpp;
#[cfg(feature = "rebuild-grammar")]
mod messages_hpp;

#[cfg(feature = "rebuild-grammar")]
pub(crate) fn codegen() {
    messages_hpp::codegen();
    messages_cpp::codegen();
}

#[cfg(not(feature = "rebuild-grammar"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
