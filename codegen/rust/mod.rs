mod finder;
mod messages;
mod nodes;
mod reserved_words;
mod visitor;

pub(crate) fn codegen() {
    messages::codegen();
    nodes::codegen();
    reserved_words::codegen();
    visitor::codegen();
    finder::codegen();
}
