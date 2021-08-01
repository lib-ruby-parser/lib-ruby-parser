#[cfg(feature = "rebuild-grammar")]
pub(crate) mod helpers;

#[cfg(feature = "rebuild-grammar")]
mod messages_c;
#[cfg(feature = "rebuild-grammar")]
mod messages_h;

#[cfg(feature = "rebuild-grammar")]
pub(crate) fn codegen() {
    messages_h::codegen();
    messages_c::codegen();
}

#[cfg(not(feature = "rebuild-grammar"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
