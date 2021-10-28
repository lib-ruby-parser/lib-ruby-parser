pub(crate) mod rust;
pub(crate) mod shared;

use lib_ruby_parser_nodes::template::TemplateFns;

pub(crate) fn build() -> TemplateFns {
    shared::build() + rust::build()
}

macro_rules! default_fns {
    () => {{
        fn generated_by(_: &lib_ruby_parser_nodes::template::GlobalContext) -> String {
            file!().to_string()
        }
        fn generated_by_for_node(_: &lib_ruby_parser_nodes::Node) -> String {
            file!().to_string()
        }
        let mut fns = $crate::codegen::fns::build();
        fns.register::<GlobalContext, F::Helper>("generated-by", generated_by);
        fns.register::<lib_ruby_parser_nodes::Node, F::Helper>(
            "generated-by",
            generated_by_for_node,
        );
        fns
    }};
}
pub(crate) use default_fns;
