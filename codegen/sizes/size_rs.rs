use super::sizes;

pub(crate) fn codegen() {
    let contents = sizes()
        .into_iter()
        .map(|size| {
            format!(
                "pub(crate) const {name}: usize = {size};",
                name = size.name,
                size = size.size
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    println!("Generating sizes.rs");
    std::fs::write("src/containers/size.rs", contents).unwrap();
}
