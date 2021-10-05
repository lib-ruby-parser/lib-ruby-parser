use lib_ruby_parser_nodes::template::*;

const TEMPLATE: &str = "// This file is autogenerated by <helper generated-by>

#include \"bindings.h\"
#include <stdio.h>

// Node constructors
<each-node><dnl>
<helper constructor-sig>
{
    LIB_RUBY_PARSER_Node node = {
        .tag = <helper node-c-enum-variant-name>,
        .as = {
            .<helper node-c-union-member-name> = {
<each-node-field><dnl>
                .<helper node-field-c-name> = <helper node-field-c-unpack-fn-name>(<helper node-field-c-name>_blob),
</each-node-field><dnl>
            }
        }
    };
    return PACK_Node(node);
}
</each-node><dnl>

// Node variant predicates
<each-node><dnl>
<helper variant-predicate-sig>
{
    LIB_RUBY_PARSER_Node *self = (LIB_RUBY_PARSER_Node *)self_blob;
    return self->tag == <helper node-c-enum-variant-name>;
}
</each-node><dnl>

// Node variant getter
<each-node><dnl>
<helper variant-getter-sig>
{
    LIB_RUBY_PARSER_Node *self = (LIB_RUBY_PARSER_Node *)self_blob;
    if (self->tag != <helper node-c-enum-variant-name>) {
        return NULL;
    }
    return (LIB_RUBY_PARSER_<helper node-camelcase-name>_BLOB *)(&(self->as.<helper node-c-union-member-name>));
}
</each-node><dnl>

// Node field getters
<each-node><dnl>
/* of <helper node-camelcase-name> */
<each-node-field><dnl>
<helper field-getter-sig>
{
    LIB_RUBY_PARSER_<helper node-camelcase-name> *self = (LIB_RUBY_PARSER_<helper node-camelcase-name> *)self_blob;
    <helper node-field-c-field-type>* field = &(self-><helper node-field-c-name>);
    return (<helper node-field-c-blob-type> *)field;
}
</each-node-field><dnl>
</each-node><dnl>

// Node field setters
<each-node><dnl>
/* of <helper node-camelcase-name> */
<each-node-field><dnl>
<helper field-setter-sig>
{
    LIB_RUBY_PARSER_<helper node-camelcase-name>* self = (LIB_RUBY_PARSER_<helper node-camelcase-name> *)self_blob;
    <helper node-field-drop-fn-name>(&(self-><helper node-field-c-name>));
    self-><helper node-field-c-name> = <helper node-field-c-unpack-fn-name>(<helper node-field-c-name>_blob);
}
</each-node-field><dnl>
</each-node><dnl>

// into_variant fns
<each-node><dnl>
<helper into-variant-sig>
{
    LIB_RUBY_PARSER_Node self = UNPACK_Node(self_blob);
    LIB_RUBY_PARSER_<helper node-camelcase-name> variant = self.as.<helper node-c-union-member-name>;
    return PACK_<helper node-camelcase-name>(variant);
}
</each-node><dnl>

// into_internal fns
<each-node><dnl>
<helper into-internal-sig>
{
    LIB_RUBY_PARSER_<helper node-camelcase-name> self = UNPACK_<helper node-camelcase-name>(self_blob);
    Internal<helper node-camelcase-name> internal = {
<each-node-field><dnl>
        .<helper node-field-c-name> = <helper node-field-c-pack-fn-name>(self.<helper node-field-c-name>),
</each-node-field><dnl>
    };
    return internal;
}
</each-node><dnl>

// variant drop fns
<each-node><dnl>
<helper drop-variant-sig>
{
    LIB_RUBY_PARSER_<helper node-camelcase-name> *self = (LIB_RUBY_PARSER_<helper node-camelcase-name> *)self_blob;
    LIB_RUBY_PARSER_drop_node_<helper node-lower-name>(self);
}
</each-node><dnl>

void lib_ruby_parser__external__node__drop(LIB_RUBY_PARSER_Node_BLOB* self_blob)
{
    LIB_RUBY_PARSER_Node *self = (LIB_RUBY_PARSER_Node *)self_blob;
    LIB_RUBY_PARSER_drop_node(self);
}
";

pub(crate) fn codegen() {
    let template = TemplateRoot::new(TEMPLATE).unwrap();
    let mut fns = crate::codegen::fns::default_fns!();
    fns.register_helper(
        "constructor-sig",
        lib_ruby_parser_bindings::helpers::nodes::constructor::sig,
    );
    fns.register_helper(
        "variant-predicate-sig",
        lib_ruby_parser_bindings::helpers::nodes::variant_predicate::sig,
    );
    fns.register_helper(
        "variant-getter-sig",
        lib_ruby_parser_bindings::helpers::nodes::variant_getter::sig,
    );
    fns.register_helper(
        "field-getter-sig",
        lib_ruby_parser_bindings::helpers::nodes::field_getter::sig,
    );
    fns.register_helper(
        "field-setter-sig",
        lib_ruby_parser_bindings::helpers::nodes::field_setter::sig,
    );
    fns.register_helper(
        "into-variant-sig",
        lib_ruby_parser_bindings::helpers::nodes::into_variant::sig,
    );
    fns.register_helper(
        "into-internal-sig",
        lib_ruby_parser_bindings::helpers::nodes::into_internal::sig,
    );
    fns.register_helper(
        "drop-variant-sig",
        lib_ruby_parser_bindings::helpers::nodes::drop_variant::sig,
    );

    let contents = template.render(ALL_DATA, &fns);
    std::fs::write("external/c/bindings_nodes.c", contents).unwrap();
}
