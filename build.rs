extern crate rust_bison_skeleton;
use std::path::Path;

fn main() {
    match rust_bison_skeleton::process_bison_file(&Path::new("src/parser.y")) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Failed to generate grammar.\n{:#?}", err);
            std::process::exit(1);
        }
    }
}
