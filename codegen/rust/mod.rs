#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod reserved_words;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod visitor;

pub(crate) mod size_rs;

#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) fn codegen() {
    reserved_words::codegen();
    visitor::codegen();
    messages::codegen();
    nodes::codegen();
}

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
