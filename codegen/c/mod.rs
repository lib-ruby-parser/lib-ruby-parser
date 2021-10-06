mod options;

mod bindings_messages_c;
mod messages_c;
mod messages_h;

mod bindings_nodes_c;
mod nodes_c;
mod nodes_h;

mod bindings_h;

mod blobs_gen_h;
mod sizes_gen_h;

pub(crate) fn codegen() {
    let options = options::codegen_options();

    bindings_h::codegen(&options);

    bindings_messages_c::codegen();

    messages_h::codegen();
    messages_c::codegen();

    bindings_nodes_c::codegen();

    nodes_h::codegen();
    nodes_c::codegen();

    blobs_gen_h::codegen();
    sizes_gen_h::codegen();
}
