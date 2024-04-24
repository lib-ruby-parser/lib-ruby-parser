pub(crate) fn print_build_info() {
    if cfg!(feature = "onig") {
        eprintln!("Using 'onig' feature")
    }
    std::process::exit(0);
}
