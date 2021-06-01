#[cfg(feature = "link-with-external-c-structures")]
pub(crate) fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-c");
    println!("cargo:rerun-if-changed=external/libstructures-c.a");
}

#[cfg(feature = "link-with-external-cpp-structures")]
pub(crate) fn link_with_external_structures() {
    println!("cargo:rustc-link-search=external");
    println!("cargo:rustc-link-lib=static=structures-cpp");
    println!("cargo:rerun-if-changed=external/libstructures-cpp.a");

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
