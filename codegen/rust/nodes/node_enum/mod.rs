#[cfg(feature = "lib-ruby-parser-bindings")]
mod external;
mod native;

pub(crate) fn codegen() {
    #[cfg(feature = "lib-ruby-parser-bindings")]
    external::codegen();
    native::codegen();
}
