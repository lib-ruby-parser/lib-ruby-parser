extern crate rust_bison_skeleton;
use std::path::Path;

pub fn generate_parser_y() {
    println!("cargo:rerun-if-changed=src/parser.y");

    match rust_bison_skeleton::process_bison_file(&Path::new("src/parser.y")) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}
