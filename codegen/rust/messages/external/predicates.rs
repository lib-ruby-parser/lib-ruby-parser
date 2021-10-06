use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is auto-generated by <helper generated-by>

use crate::blobs::Blob;
use super::DiagnosticMessage;

extern \"C\" {
<each-message><dnl>
    fn <helper extern-predicate-name>(blob: *const Blob<DiagnosticMessage>) -> bool;
</each-message><dnl>
}

impl DiagnosticMessage {
<each-message><dnl>
    /// Returns true if current variant is <helper message-camelcase-name>
    pub fn is_<helper message-lower-name>(&self) -> bool {
        unsafe { <helper extern-predicate-name>(&self.blob) }
    }
</each-message><dnl>
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();

    fns.register_helper(
        "extern-predicate-name",
        lib_ruby_parser_bindings::helpers::messages::variant_predicate::name,
    );

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("src/error/message/external/predicates.rs", contents).unwrap();
}
