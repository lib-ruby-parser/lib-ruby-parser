#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) mod helpers;

#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_messages_c;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_messages_h;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_nodes_c;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod bindings_nodes_h;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages_c;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod messages_h;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes_c;
#[cfg(feature = "lib-ruby-parser-nodes")]
mod nodes_h;

#[cfg(feature = "lib-ruby-parser-nodes")]
pub(crate) fn codegen() {
    nodes_h::codegen();
    nodes_c::codegen();

    bindings_nodes_h::codegen();
    bindings_nodes_c::codegen();

    messages_h::codegen();
    messages_c::codegen();

    bindings_messages_h::codegen();
    bindings_messages_c::codegen();
}

#[cfg(not(feature = "lib-ruby-parser-nodes"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
