mod constructors;
mod enum_;
mod getters;
mod predicates;

pub(crate) fn codegen() {
    enum_::codegen();
    constructors::codegen();
    predicates::codegen();
    getters::codegen();
}
