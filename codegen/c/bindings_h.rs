use lib_ruby_parser_bindings::{generate, Options};

pub(crate) fn codegen(options: &Options) {
    let contents = generate(&options);

    std::fs::write("external/c/bindings.h", contents).unwrap()
}
