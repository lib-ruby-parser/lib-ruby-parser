use lib_ruby_parser_nodes::{
    reexports::liquid::value,
    LiquidTemplate,
};

fn main() {
    /* Node test helper */
    let template = LiquidTemplate::new("codegen/loc_name.liquid")
    .with_global(
        "loc_names",
        value!(vec![
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
        ]),
    );
    let rendered = template.render();
    std::fs::write("src/test_helpers/loc_matcher/loc_name_gen.rs", rendered).unwrap();
}
