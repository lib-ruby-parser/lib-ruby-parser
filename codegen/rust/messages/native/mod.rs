mod constructors;
mod enum_;
mod predicates;
mod variants;

pub(crate) fn codegen() {
    enum_::codegen();
    constructors::codegen();
    predicates::codegen();
    variants::codegen();
}
