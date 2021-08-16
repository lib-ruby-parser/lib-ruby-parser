mod external;
mod native;
mod tests;

pub(crate) fn codegen() {
    external::codegen();
    native::codegen();
    tests::codegen()
}
