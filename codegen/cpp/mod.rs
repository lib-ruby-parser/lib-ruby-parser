#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) mod helpers;

#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_messages_cpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_messages_hpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages_cpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages_hpp;

#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_nodes_cpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_nodes_hpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes_cpp;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes_hpp;

#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) fn codegen() {
    bindings_messages_hpp::codegen();
    bindings_messages_cpp::codegen();

    messages_hpp::codegen();
    messages_cpp::codegen();

    bindings_nodes_hpp::codegen();
    bindings_nodes_cpp::codegen();

    nodes_hpp::codegen();
    nodes_cpp::codegen();
}

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
