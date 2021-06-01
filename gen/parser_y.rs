#[cfg(feature = "rust-bison-skeleton")]
extern crate rust_bison_skeleton;

#[cfg(feature = "rust-bison-skeleton")]
pub(crate) fn generate_parser_y() {
    use std::path::Path;
    const PARSER_Y: &str = "src/parser.y";

    println!("cargo:rerun-if-changed={}", PARSER_Y);

    match rust_bison_skeleton::process_bison_file(&Path::new(PARSER_Y)) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}

#[cfg(not(feature = "rust-bison-skeleton"))]
pub(crate) fn generate_parser_y() {
    println!("Skipping generating parser.rs")
}
