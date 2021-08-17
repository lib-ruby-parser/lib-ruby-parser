mod c;
mod cpp;
mod rust;
mod y;

pub(crate) fn codegen() {
    y::codegen();
    rust::codegen();
    c::codegen();
    cpp::codegen();
    rust::size_rs::codegen();
}
