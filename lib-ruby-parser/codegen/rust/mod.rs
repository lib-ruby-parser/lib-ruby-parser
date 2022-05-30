mod reserved_words;

use lib_ruby_parser_nodes::{
    helpers::{camelcase_to_snakecase, escape_rust_keyword},
    reexports::liquid::value,
    LiquidTemplate,
};

fn render<F>(template_path: &str, output_path: &str, f: F)
where
    F: Fn(LiquidTemplate) -> LiquidTemplate,
{
    let template = LiquidTemplate::new(template_path);
    let template = f(template);
    let rendered = template.render();
    std::fs::write(output_path, rendered).unwrap()
}

fn no_template_options(template: LiquidTemplate) -> LiquidTemplate {
    template
}

pub(crate) fn codegen() {
    /* Messages */
    render(
        "codegen/rust/messages.liquid",
        "src/error/messages/message_enum.rs",
        no_template_options,
    );

    /* Individual node files */
    let nodes = lib_ruby_parser_nodes::nodes();
    std::fs::create_dir_all("src/nodes/types").unwrap();
    for node in nodes.iter() {
        let filename = escape_rust_keyword(&camelcase_to_snakecase(node.camelcase_name));
        render(
            "codegen/rust/node_file.liquid",
            &format!("src/nodes/types/{}.rs", filename),
            |template| template.with_global("node", value!(node.to_owned())),
        );
    }
    /* Node mod.rs */
    render(
        "codegen/rust/node_mod.liquid",
        "src/nodes/types/mod.rs",
        no_template_options,
    );
    /* Node enum */
    render(
        "codegen/rust/node_enum.liquid",
        "src/nodes/node_enum.rs",
        no_template_options,
    );

    reserved_words::codegen();

    /* Visitor API */
    render(
        "codegen/rust/visitor.liquid",
        "src/traverse/visitor/visit_gen.rs",
        no_template_options,
    );

    /* Finder API */
    render(
        "codegen/rust/finder.liquid",
        "src/traverse/finder/finder_gen.rs",
        no_template_options,
    );
}
