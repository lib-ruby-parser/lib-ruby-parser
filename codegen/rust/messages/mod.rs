mod native;
mod tests;

pub(crate) fn codegen() {
    native::codegen();
    tests::codegen()
}
