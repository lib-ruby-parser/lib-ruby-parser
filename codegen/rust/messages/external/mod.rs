mod constructors;
mod impl_clone;
mod impl_debug;
mod impl_partial_eq;
mod predicates;
mod variants;

pub(crate) fn codegen() {
    constructors::codegen();
    impl_clone::codegen();
    impl_debug::codegen();
    impl_partial_eq::codegen();
    predicates::codegen();
    variants::codegen();
}
