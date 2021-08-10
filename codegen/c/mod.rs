#[cfg(feature = "compile-with-external-structures")]
pub(crate) mod helpers;

#[cfg(feature = "compile-with-external-structures")]
mod bindings_messages_c;
#[cfg(feature = "compile-with-external-structures")]
mod bindings_messages_h;
#[cfg(feature = "compile-with-external-structures")]
mod bindings_nodes_c;
#[cfg(feature = "compile-with-external-structures")]
mod bindings_nodes_h;
#[cfg(feature = "compile-with-external-structures")]
mod messages_c;
#[cfg(feature = "compile-with-external-structures")]
mod messages_h;
#[cfg(feature = "compile-with-external-structures")]
mod nodes_c;
#[cfg(feature = "compile-with-external-structures")]
mod nodes_h;

#[cfg(feature = "compile-with-external-structures")]
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

#[cfg(not(feature = "compile-with-external-structures"))]
pub(crate) fn codegen() {
    println!("Skipping codegen in {}", file!())
}
