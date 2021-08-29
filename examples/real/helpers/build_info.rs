pub(crate) struct BuildInfo {}

impl BuildInfo {
    pub fn print() {
        if cfg!(feature = "onig") {
            println!("Using 'onig' feature")
        }
        if cfg!(feature = "compile-with-external-structures") {
            println!("Using 'compile-with-external-structures' feature")
        } else {
            println!("Using Rust structures")
        }
        std::process::exit(0);
    }
}
