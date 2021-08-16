mod messages;
mod nodes;
mod reserved_words;
mod visitor;

pub(crate) mod size_rs;

pub(crate) fn codegen() {
    reserved_words::codegen();
    visitor::codegen();
    messages::codegen();
    nodes::codegen();
}
