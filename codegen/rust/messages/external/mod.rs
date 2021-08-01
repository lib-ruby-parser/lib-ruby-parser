mod constructors;
mod getters;
mod impl_clone;
mod impl_debug;
mod impl_drop;
mod impl_partial_eq;
mod predicates;

pub(crate) fn codegen() {
    constructors::codegen();
    getters::codegen();
    impl_clone::codegen();
    impl_debug::codegen();
    impl_drop::codegen();
    impl_partial_eq::codegen();
    predicates::codegen();
}
