mod gen;
use gen::{generate_nodes, generate_parser_y};

#[cfg(feature = "link-external-c-structures")]
fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-c");
}

#[cfg(feature = "link-external-cpp-structures")]
fn link_with_external_structures() {
    println!("cargo:rustc-link-lib=dylib=c++");

    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-cpp");
}

#[cfg(not(any(
    feature = "link-external-c-structures",
    feature = "link-external-cpp-structures"
)))]
fn link_with_external_structures() {
    // noop
}

fn main() {
    generate_parser_y();
    generate_nodes();

    link_with_external_structures();
}
