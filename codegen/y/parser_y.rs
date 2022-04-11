extern crate rust_bison_skeleton;

pub(crate) fn codegen() {
    use std::path::Path;
    const PARSE_Y: &str = "src/parser/parse.y";

    println!("cargo:rerun-if-changed={}", PARSE_Y);
    println!("Generating parse.rs");

    match rust_bison_skeleton::process_bison_file(Path::new(PARSE_Y)) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}
