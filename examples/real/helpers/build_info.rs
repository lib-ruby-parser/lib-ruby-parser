pub(crate) struct BuildInfo {}

impl BuildInfo {
    pub fn print() {
        if cfg!(feature = "onig") {
            println!("Using 'onig' feature")
        }
        std::process::exit(0);
    }
}
