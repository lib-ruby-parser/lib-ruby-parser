mod external;
mod native;

pub(crate) fn codegen() {
    external::codegen();
    native::codegen();
}
