#[cfg(feature = "rebuild-grammar")]
mod messages;
#[cfg(feature = "rebuild-grammar")]
mod nodes;
#[cfg(feature = "rebuild-grammar")]
mod reserved_words;
#[cfg(feature = "rebuild-grammar")]
mod visitor;

pub(crate) mod size_rs;

#[cfg(feature = "rebuild-grammar")]
pub(crate) fn codegen() {
    reserved_words::codegen();
    visitor::codegen();
    messages::codegen();
    nodes::codegen();
}

#[cfg(not(feature = "rebuild-grammar"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
