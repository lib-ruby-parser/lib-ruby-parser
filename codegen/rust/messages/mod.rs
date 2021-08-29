#[cfg(feature = "lib-ruby-parser-bindings")]
mod external;
mod native;
mod tests;

pub(crate) fn codegen() {
    #[cfg(feature = "lib-ruby-parser-bindings")]
    external::codegen();
    native::codegen();
    tests::codegen()
}
