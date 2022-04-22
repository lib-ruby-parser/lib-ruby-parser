use lib_ruby_parser_nodes::{reexports::liquid::value, LiquidTemplate};

const LOC_NAMES: &[&str] = &[
    "begin",
    "end",
    "expression",
    "keyword",
    "name",
    "assignment",
    "colon",
    "double_colon",
    "else",
    "heredoc_body",
    "operator",
    "selector",
    "assoc",
    "question",
    "heredoc_end",
];

pub(crate) fn codegen() {
    let contents = LiquidTemplate::new("codegen/rust/nodes/loc_name.liquid")
        .with_global("loc_names", value!(LOC_NAMES.to_owned()))
        .render();
    std::fs::write("src/test_helpers/loc_matcher/loc_name_gen.rs", contents).unwrap();
}
