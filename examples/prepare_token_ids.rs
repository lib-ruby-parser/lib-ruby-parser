#[cfg(feature = "prepare-token-ids-example")]
fn main() {
    #[derive(Debug, Default)]
    struct TokenId {
        comment: String,
        name: String,
        value: String,
    }

    const TMP_BINDINGS: &str = "target/tmp_bindings.h";

    cbindgen::Builder::new()
        .with_crate(".")
        .with_parse_deps(false)
        .with_no_includes()
        .include_item("Lexer")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(TMP_BINDINGS);

    let bindings = std::fs::read_to_string(TMP_BINDINGS).unwrap();

    std::fs::remove_file(TMP_BINDINGS).unwrap();

    let mut token_ids = vec![];
    let mut token_id = TokenId::default();

    for line in bindings.lines() {
        if line.starts_with("/// Token") {
            // token comment line
            token_id.comment = line.replace("///", "");
        } else if line.starts_with("static const int32_t Lexer_") {
            let line = line
                .replace("static const int32_t Lexer_", "")
                .replace(";", "");

            let parts = line.split(" = ").collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);
            token_id.name = parts[0].to_string();
            token_id.value = parts[1].to_string();

            token_ids.push(std::mem::take(&mut token_id));
        }
    }

    std::fs::write("target/tokens.rs", format!("{:?}", token_ids)).unwrap();
}

#[cfg(not(feature = "prepare-token-ids-example"))]
pub fn main() {
    println!(
        "'prepare_token_ids' example must be executed with 'prepare-token-ids-example' feature"
    )
}
