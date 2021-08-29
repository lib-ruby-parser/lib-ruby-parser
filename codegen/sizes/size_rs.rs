pub(crate) fn codegen() {
    println!("cargo:rerun-if-env-changed=LIB_RUBY_PARSER_SIZES_FILEPATH");
    let sizes_filepath = env!("LIB_RUBY_PARSER_SIZES_FILEPATH");

    let sizes =
        std::fs::read_to_string(sizes_filepath).expect("failed to read file with struct sizes");

    let contents = sizes.lines().map(|line| line.replace("LIB_RUBY_PARSER_", "")).map(|line| {
        let parts = line.split("=").collect::<Vec<_>>();
        if parts.len() != 2 {
            panic!("Wrong format of the sizes file. Must be LIB_RUBY_PARSER_<STRUCT_NAME>_SIZE=<SIZE>");
        }
        let name = parts[0];
        let size = parts[1];

        format!("pub(crate) const {name}: usize = {size};", name = name, size = size)
    }).collect::<Vec<_>>().join("\n");

    println!("Generating sizes.rs");
    std::fs::write("src/containers/size.rs", contents).unwrap();
}
