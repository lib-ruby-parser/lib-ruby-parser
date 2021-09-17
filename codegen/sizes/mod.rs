mod blobs;
mod size_rs;

pub(crate) fn codegen() {
    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_SIZES_FILEPATH");

    size_rs::codegen();
    blobs::codegen()
}

pub(crate) struct Size {
    pub(crate) name: String,
    pub(crate) size: String,
}

pub(crate) fn sizes() -> Vec<Size> {
    let sizes_filepath = env!("LIB_RUBY_PARSER_SIZES_FILEPATH");

    let sizes =
        std::fs::read_to_string(sizes_filepath).expect("failed to read file with struct sizes");

    sizes.lines().map(|line| line.replace("LIB_RUBY_PARSER_", "")).map(|line| {
        let parts = line.split('=').collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Wrong format of the sizes file. Must be LIB_RUBY_PARSER_<STRUCT_NAME>_SIZE=<SIZE>");
        }
        let name = parts[0].to_owned();
        let size = parts[1].to_owned();

        Size { name, size }
    }).collect()
}
