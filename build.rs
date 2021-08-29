mod codegen;

fn main() {
    codegen::codegen();

    link_with_external_structures();
}

#[cfg(feature = "link-with-external-c-structures")]
pub(crate) fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external/c");
    println!("cargo:rustc-link-lib=static=structures");
    println!("cargo:rerun-if-changed=external/c/libstructures.a");
}

#[cfg(feature = "link-with-external-cpp-structures")]
pub(crate) fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external/cpp");
    println!("cargo:rustc-link-lib=static=structures");
    println!("cargo:rerun-if-changed=external/cpplibstructures.a");

    println!("cargo:rustc-link-lib=dylib=c++");
    if cfg!(target_os = "linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    }
}

#[cfg(not(any(
    feature = "link-with-external-c-structures",
    feature = "link-with-external-cpp-structures"
)))]
pub(crate) fn link_with_external_structures() {
    println!("Skipping linking with external structures")
}
