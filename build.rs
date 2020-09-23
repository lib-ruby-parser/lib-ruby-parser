extern crate rust_bison_skeleton;
use std::path::Path;

fn main() {
    rust_bison_skeleton::process_bison_file(&Path::new("src/parser.y")).unwrap()
}
