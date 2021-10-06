pub(crate) mod helpers;
mod options;

mod bindings_messages_cpp;
mod messages_cpp;
mod messages_hpp;

mod bindings_nodes_cpp;
mod nodes_cpp;
mod nodes_hpp;

mod bindings_hpp;

mod blobs_gen_hpp;
mod sizes_gen_hpp;

pub(crate) fn codegen() {
    let options = options::codegen_options();

    bindings_hpp::codegen(&options);

    bindings_messages_cpp::codegen();

    messages_hpp::codegen();
    messages_cpp::codegen();

    bindings_nodes_cpp::codegen();

    nodes_hpp::codegen();
    nodes_cpp::codegen();

    blobs_gen_hpp::codegen();
    sizes_gen_hpp::codegen();
}
